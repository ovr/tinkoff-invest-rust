use tcs::{
    instruments_service_client::InstrumentsServiceClient,
    market_data_service_client::MarketDataServiceClient,
    market_data_stream_service_client::MarketDataStreamServiceClient,
    operations_service_client::OperationsServiceClient,
    orders_stream_service_client::OrdersStreamServiceClient,
    sandbox_service_client::SandboxServiceClient,
    stop_orders_service_client::StopOrdersServiceClient, users_service_client::UsersServiceClient,
};
use tonic::{
    codegen::InterceptedService,
    service::Interceptor,
    transport::{self, Channel, ClientTlsConfig},
    Status,
};

pub mod tcs;

#[derive(Debug)]
pub struct TinkoffInvestService {
    token: String,
}

#[derive(Debug)]
pub struct DefaultInterceptor {
    token: String,
}

impl Interceptor for DefaultInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let mut req = request;
        req.metadata_mut().append(
            "authorization",
            format!("bearer {}", self.token).parse().unwrap(),
        );
        req.metadata_mut().append(
            "x-tracking-id",
            uuid::Uuid::new_v4().to_string().parse().unwrap(),
        );
        req.metadata_mut()
            .append("x-app-name", "ovr.tinkoffInvestRust".parse().unwrap());

        Ok(req)
    }
}

#[derive(Debug)]
pub enum TIError {
    /// Error's that originate from the client or server;
    TransportError(transport::Error),
    /// A gRPC status describing the result of an RPC call.
    StatusError(tonic::Status),
}

impl From<transport::Error> for TIError {
    fn from(e: transport::Error) -> Self {
        TIError::TransportError(e)
    }
}

impl From<tonic::Status> for TIError {
    fn from(e: tonic::Status) -> Self {
        TIError::StatusError(e)
    }
}

pub type TIResult<T> = std::result::Result<T, TIError>;

impl TinkoffInvestService {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn create_channel(&self) -> TIResult<Channel> {
        let tls = ClientTlsConfig::new();

        let channel = Channel::from_static("https://invest-public-api.tinkoff.ru:443/")
            .tls_config(tls)?
            .connect()
            .await?;

        Ok(channel)
    }

    pub async fn users(
        &self,
        channel: Channel,
    ) -> TIResult<UsersServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = UsersServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn operations(
        &self,
        channel: Channel,
    ) -> TIResult<OperationsServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = OperationsServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn instruments(
        &self,
        channel: Channel,
    ) -> TIResult<InstrumentsServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = InstrumentsServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn marketdata(
        &self,
        channel: Channel,
    ) -> TIResult<MarketDataServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = MarketDataServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn marketdata_stream(
        &self,
        channel: Channel,
    ) -> TIResult<MarketDataStreamServiceClient<InterceptedService<Channel, DefaultInterceptor>>>
    {
        let client = MarketDataStreamServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn orders_stream(
        &self,
        channel: Channel,
    ) -> TIResult<OrdersStreamServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = OrdersStreamServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn stop_orders(
        &self,
        channel: Channel,
    ) -> TIResult<StopOrdersServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = StopOrdersServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }

    pub async fn sandbox(
        &self,
        channel: Channel,
    ) -> TIResult<SandboxServiceClient<InterceptedService<Channel, DefaultInterceptor>>> {
        let client = SandboxServiceClient::with_interceptor(
            channel,
            DefaultInterceptor {
                token: self.token.clone(),
            },
        );

        Ok(client)
    }
}
