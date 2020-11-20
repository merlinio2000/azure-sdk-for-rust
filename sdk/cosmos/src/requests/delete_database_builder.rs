use crate::prelude::*;
use crate::responses::DeleteDatabaseResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    database_client: &'a dyn DatabaseClient<C>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, C> DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    pub(crate) fn new(database_client: &'a dyn DatabaseClient<C>) -> DeleteDatabaseBuilder<'a, C> {
        DeleteDatabaseBuilder {
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, C> DatabaseClientRequired<'a, C> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn database_client(&self) -> &'a dyn DatabaseClient<C> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> UserAgentOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, C> ActivityIdOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, C> ConsistencyLevelOption<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, C> UserAgentSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C> ActivityIdSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, C> ConsistencyLevelSupport<'a> for DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    type O = DeleteDatabaseBuilder<'a, C>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        DeleteDatabaseBuilder {
            database_client: self.database_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteDatabaseBuilder<'a, C>
where
    C: CosmosClient,
{
    pub async fn execute(&self) -> Result<DeleteDatabaseResponse, CosmosError> {
        trace!("DeleteDatabaseResponse::execute called");

        let request = self
            .database_client()
            .prepare_request_with_database_name(http::Method::DELETE);

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(EMPTY_BODY.as_ref())?;

        trace!("request prepared == {:?}", request);

        Ok(self
            .database_client()
            .http_client()
            .execute_request_check_status(request, StatusCode::NO_CONTENT)
            .await?
            .try_into()?)
    }
}
