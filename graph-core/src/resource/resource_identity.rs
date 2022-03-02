use inflector::Inflector;
use std::convert::AsRef;

/// Comprises both top level and second level resources.
/// These are not generated from OpenApi, except for top level resources,
/// and mostly consist of Apis that the project currently has generated.
#[derive(
    AsRefStr,
    Copy,
    Clone,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum ResourceIdentity {
    Activities,
    Admin,
    AgreementAcceptances,
    Agreements,
    AppCatalogs,
    Application,
    ApplicationTemplates,
    Applications,
    Attachments,
    AuditLogs,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Buckets,
    Branding,
    Calendar,
    CalendarGroup,
    CalendarGroups,
    CalendarView,
    CalendarViews,
    Calendars,
    CallRecord,
    CallRecords,
    Calls,
    CertificateBasedAuthConfiguration,
    Chats,
    ChildFolders,
    Communications,
    Compliance,
    Connections,
    ContactFolders,
    Contacts,
    ContentTypes,
    Contracts,
    Conversations,
    DataPolicyOperations,
    DeviceAppManagement,
    DeviceManagement,
    Devices,
    Directory,
    DirectoryObjects,
    DirectoryRoleTemplates,
    DirectoryRoles,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    Education,
    Event,
    Events,
    External,
    ExtendedProperties,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    HealthOverviews,
    HistoryItems,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    InferenceClassification,
    InformationProtection,
    Insights,
    Instances,
    Invitations,
    Items,
    List,
    Lists,
    Localizations,
    MailFolders,
    ManagedDevices,
    Me,
    Messages,
    Notebooks,
    Oauth2PermissionGrants,
    Onenote,
    OnlineMeetings,
    OrgContact,
    Organization,
    Outlook,
    Pages,
    ParentNotebook,
    ParentSection,
    ParentSectionGroup,
    PermissionGrants,
    Places,
    Planner,
    Plans,
    Policies,
    Posts,
    Print,
    Privacy,
    Reports,
    RoleManagement,
    ScopedRoleMemberships,
    SchemaExtensions,
    Search,
    SectionGroups,
    Sections,
    Security,
    ServiceAnnouncement,
    ServicePrincipals,
    Sessions,
    Settings,
    Shares,
    Sites,
    Solutions,
    SubscribedSkus,
    Subscriptions,
    Tasks,
    Teams,
    TeamsTemplates,
    Teamwork,
    Threads,
    Users,
    Workbooks,
}

impl ToString for ResourceIdentity {
    fn to_string(&self) -> String {
        self.as_ref().to_camel_case()
    }
}

impl Default for ResourceIdentity {
    fn default() -> Self {
        ResourceIdentity::Me
    }
}

impl ResourceIdentity {
    pub fn enum_string(&self) -> String {
        format!("ResourceIdentity::{:#?}", self)
    }
}

/// Top level resources are the names for the first or beginning part of a URI path.
/// These are generated from the OpenApi config.
#[derive(
    AsRefStr,
    Copy,
    Clone,
    Eq,
    PartialEq,
    EnumString,
    EnumIter,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "camelCase")]
pub enum TopLevelResource {
    Admin,
    AgreementAcceptances,
    Agreements,
    AppCatalogs,
    ApplicationTemplates,
    Applications,
    AuditLogs,
    AuthenticationMethodConfigurations,
    AuthenticationMethodsPolicy,
    Branding,
    CertificateBasedAuthConfiguration,
    Chats,
    Communications,
    Compliance,
    Connections,
    Contacts,
    Contracts,
    DataPolicyOperations,
    DeviceAppManagement,
    DeviceManagement,
    Devices,
    Directory,
    DirectoryObjects,
    DirectoryRoleTemplates,
    DirectoryRoles,
    DomainDnsRecords,
    Domains,
    Drive,
    Drives,
    Education,
    External,
    GroupLifecyclePolicies,
    GroupSettingTemplates,
    GroupSettings,
    Groups,
    Identity,
    IdentityGovernance,
    IdentityProtection,
    IdentityProviders,
    InformationProtection,
    Invitations,
    Localizations,
    Me,
    Oauth2PermissionGrants,
    Organization,
    PermissionGrants,
    Places,
    Planner,
    Policies,
    Print,
    Privacy,
    Reports,
    RoleManagement,
    SchemaExtensions,
    ScopedRoleMemberships,
    Search,
    Security,
    ServicePrincipals,
    Shares,
    Sites,
    Solutions,
    SubscribedSkus,
    Subscriptions,
    Teams,
    TeamsTemplates,
    Teamwork,
    Users,
    Workbooks,
}

impl ToString for TopLevelResource {
    fn to_string(&self) -> String {
        self.as_ref().to_camel_case()
    }
}

impl Default for TopLevelResource {
    fn default() -> Self {
        TopLevelResource::Me
    }
}
