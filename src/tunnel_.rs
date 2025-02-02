use core::{error::Error, time::Duration};

use abs_buff::{x_deps::abs_sync, TrBuffIterRead, TrBuffIterWrite};
use abs_sync::cancellation::TrMayCancel;

use crate::port_::TrPort;

pub trait TrDescriptor {
    type Data: Sized;
    type Port: TrPort;
    type Message<'f>: TrBuffIterRead<Self::Data> where Self: 'f;

    fn local_port(&self) -> Self::Port;

    fn remote_port(&self) -> Self::Port;

    /// Returns the time interval since its creation or arrival.
    fn age(&self) -> Duration;

    fn message(&mut self) -> Option<Self::Message<'_>>;
}

pub trait TrListener {
    type Descriptor: TrDescriptor;
    type Err: Error;

    fn listen_async(&mut self) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Descriptor, Self::Err>>;
}

pub trait TrTelegraph {
    type Data: Sized;
    type Rx<'f>: TrBuffIterRead<Self::Data> where Self: 'f;
    type Err: Error;

    fn send_async<R>(
        &mut self,
        packet: &mut R,
    ) -> impl TrMayCancel<'_, MayCancelOutput = Result<usize, Self::Err>>
    where
        R: TrBuffIterRead<Self::Data>;

    fn receive_async(
        &mut self,
    ) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Rx<'_>, Self::Err>>;
}

/// Duplex communication with the inbound stream and outbound stream. usually
/// the product of the multiplexed tunnel of [TrMuxTunnel].
pub trait TrChannel {
    type Data: Sized;
    type Tx<'f>: 'f + TrBuffIterWrite<Self::Data> where Self: 'f;
    type Rx<'f>: 'f + TrBuffIterRead<Self::Data> where Self: 'f;

    fn split_io(&mut self) -> (Self::Tx<'_>, Self::Rx<'_>);
}

pub trait TrPortBinder {
    type Data: Sized;
    type Port: TrPort;
    type Channel: TrChannel<Data = Self::Data>;
    type Descriptor: TrDescriptor<Data = Self::Data, Port = Self::Port>;

    type Listener<'f>: TrListener<Descriptor = Self::Descriptor>
    where
        Self: 'f;

    type Telegraph<'f>: TrTelegraph<Data = Self::Data>
    where
        Self: 'f;

    type ListenErr: Error;
    type TelegraphErr: Error;
    type ChannelErr: Error;

    fn local_port(&self) -> <Self::Descriptor as TrDescriptor>::Port;

    /// Create a listener listening at the given port.
    fn listener(&mut self) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Listener<'_>, Self::ListenErr>>;

    /// Create a telegraph for sending and receiving packets.
    fn telegraph(&mut self) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Telegraph<'_>, Self::TelegraphErr>>;

    /// Initiate a channel to the remote port
    fn channel<'f, M>(
        &'f mut self,
        remote: <Self::Descriptor as TrDescriptor>::Port,
        message: &'f mut M,
    ) -> impl TrMayCancel<'f,
        MayCancelOutput = Result<Self::Channel, Self::ChannelErr>>
    where
        M: TrBuffIterWrite;
}

pub trait TrMultiplex {
    type Data: Sized;
    type Port: TrPort;
    type Channel: TrChannel<Data = Self::Data>;
    type Descriptor: TrDescriptor<Data = Self::Data, Port = Self::Port>;

    type Binder<'f>: TrPortBinder<
        Data = Self::Data,
        Port = Self::Port,
        Channel = Self::Channel,
        Descriptor = Self::Descriptor>
    where
        Self: 'f;

    /// The error associated with a port binding
    type BindErr: Error;

    /// The error associated with a descriptor
    type DescriptorErr: Error;

    /// Bind a specific or unspecified port for later usage.
    fn bind_async(
        &self,
        port: <Self::Descriptor as TrDescriptor>::Port,
    ) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Binder<'_>, Self::BindErr>>;

    fn accept_async(
        &self,
        descriptor: Self::Descriptor,
    ) -> impl TrMayCancel<'_,
        MayCancelOutput = Result<Self::Channel, Self::DescriptorErr>>;

    fn reject_async(
        &self,
        descriptor: Self::Descriptor,
    ) -> impl TrMayCancel<'_, MayCancelOutput = 
        Result<<Self::Channel as TrChannel>::Tx<'_>, Self::DescriptorErr>>;
}
