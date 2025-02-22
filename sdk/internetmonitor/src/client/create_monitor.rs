// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateMonitor`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`monitor_name(impl Into<String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::monitor_name) / [`set_monitor_name(Option<String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_monitor_name): <p>The name of the monitor. </p>
    ///   - [`resources(Vec<String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::resources) / [`set_resources(Option<Vec<String>>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_resources): <p>The resources to include in a monitor, which you provide as a set of Amazon Resource Names (ARNs).</p>  <p>You can add a combination of Amazon Virtual Private Clouds (VPCs) and Amazon CloudFront distributions, or you can add Amazon WorkSpaces directories. You can't add all three types of resources.</p> <note>   <p>If you add only VPC resources, at least one VPC must have an Internet Gateway attached to it, to make sure that it has internet connectivity.</p>  </note>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_client_token): <p>A unique, case-sensitive string of up to 64 ASCII characters that you specify to make an idempotent API request. Don't reuse the same client token for other API requests.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_tags): <p>The tags for a monitor. You can add a maximum of 50 tags in Internet Monitor.</p>
    ///   - [`max_city_networks_to_monitor(i32)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::max_city_networks_to_monitor) / [`set_max_city_networks_to_monitor(i32)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_max_city_networks_to_monitor): <p>The maximum number of city-networks to monitor for your resources. A city-network is the location (city) where clients access your application resources from and the network or ASN, such as an internet service provider (ISP), that clients access the resources through. This limit helps control billing costs.</p>  <p>To learn more, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/IMCityNetworksMaximum.html">Choosing a city-network maximum value </a> in the Amazon CloudWatch Internet Monitor section of the <i>CloudWatch User Guide</i>.</p>
    ///   - [`internet_measurements_log_delivery(InternetMeasurementsLogDelivery)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::internet_measurements_log_delivery) / [`set_internet_measurements_log_delivery(Option<InternetMeasurementsLogDelivery>)`](crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::set_internet_measurements_log_delivery): <p>Publish internet measurements for Internet Monitor to another location, such as an Amazon S3 bucket. The measurements are also published to Amazon CloudWatch Logs.</p>
    /// - On success, responds with [`CreateMonitorOutput`](crate::operation::create_monitor::CreateMonitorOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_monitor::CreateMonitorOutput::arn): <p>The Amazon Resource Name (ARN) of the monitor.</p>
    ///   - [`status(Option<MonitorConfigState>)`](crate::operation::create_monitor::CreateMonitorOutput::status): <p>The status of a monitor.</p>
    /// - On failure, responds with [`SdkError<CreateMonitorError>`](crate::operation::create_monitor::CreateMonitorError)
    pub fn create_monitor(
        &self,
    ) -> crate::operation::create_monitor::builders::CreateMonitorFluentBuilder {
        crate::operation::create_monitor::builders::CreateMonitorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
