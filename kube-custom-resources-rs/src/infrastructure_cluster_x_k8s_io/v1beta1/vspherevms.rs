// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --filename=./crd-catalog/kubernetes-sigs/cluster-api-provider-vsphere/infrastructure.cluster.x-k8s.io/v1beta1/vspherevms.yaml --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.21.1

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// VSphereVMSpec defines the desired state of VSphereVM.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "VSphereVM", plural = "vspherevms")]
#[kube(namespaced)]
#[kube(status = "VSphereVMStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct VSphereVMSpec {
    /// AdditionalDisksGiB holds the sizes of additional disks of the virtual machine, in GiB
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalDisksGiB")]
    pub additional_disks_gi_b: Option<Vec<i64>>,
    /// BiosUUID is the VM's BIOS UUID that is assigned at runtime after
    /// the VM has been created.
    /// This field is required at runtime for other controllers that read
    /// this CRD as unstructured data.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "biosUUID")]
    pub bios_uuid: Option<String>,
    /// BootstrapRef is a reference to a bootstrap provider-specific resource
    /// that holds configuration details.
    /// This field is optional in case no bootstrap data is required to create
    /// a VM.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bootstrapRef")]
    pub bootstrap_ref: Option<ObjectReference>,
    /// CloneMode specifies the type of clone operation.
    /// The LinkedClone mode is only support for templates that have at least
    /// one snapshot. If the template has no snapshots, then CloneMode defaults
    /// to FullClone.
    /// When LinkedClone mode is enabled the DiskGiB field is ignored as it is
    /// not possible to expand disks of linked clones.
    /// Defaults to LinkedClone, but fails gracefully to FullClone if the source
    /// of the clone operation has no snapshots.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneMode")]
    pub clone_mode: Option<String>,
    /// CustomVMXKeys is a dictionary of advanced VMX options that can be set on VM
    /// Defaults to empty map
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customVMXKeys")]
    pub custom_vmx_keys: Option<BTreeMap<String, String>>,
    /// Datacenter is the name or inventory path of the datacenter in which the
    /// virtual machine is created/located.
    /// Defaults to * which selects the default datacenter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// Datastore is the name or inventory path of the datastore in which the
    /// virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datastore: Option<String>,
    /// DiskGiB is the size of a virtual machine's disk, in GiB.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskGiB")]
    pub disk_gi_b: Option<i32>,
    /// Folder is the name or inventory path of the folder in which the
    /// virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    /// GuestSoftPowerOffTimeout sets the wait timeout for shutdown in the VM guest.
    /// The VM will be powered off forcibly after the timeout if the VM is still
    /// up and running when the PowerOffMode is set to trySoft.
    /// 
    /// 
    /// This parameter only applies when the PowerOffMode is set to trySoft.
    /// 
    /// 
    /// If omitted, the timeout defaults to 5 minutes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "guestSoftPowerOffTimeout")]
    pub guest_soft_power_off_timeout: Option<String>,
    /// HardwareVersion is the hardware version of the virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Check the compatibility with the ESXi version before setting the value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hardwareVersion")]
    pub hardware_version: Option<String>,
    /// MemoryMiB is the size of a virtual machine's memory, in MiB.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memoryMiB")]
    pub memory_mi_b: Option<i64>,
    /// Network is the network configuration for this machine's VM.
    pub network: VSphereVMNetwork,
    /// NumCPUs is the number of virtual processors in a virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numCPUs")]
    pub num_cp_us: Option<i32>,
    /// NumCPUs is the number of cores among which to distribute CPUs in this
    /// virtual machine.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "numCoresPerSocket")]
    pub num_cores_per_socket: Option<i32>,
    /// OS is the Operating System of the virtual machine
    /// Defaults to Linux
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// PciDevices is the list of pci devices used by the virtual machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pciDevices")]
    pub pci_devices: Option<Vec<VSphereVMPciDevices>>,
    /// PowerOffMode describes the desired behavior when powering off a VM.
    /// 
    /// 
    /// There are three, supported power off modes: hard, soft, and
    /// trySoft. The first mode, hard, is the equivalent of a physical
    /// system's power cord being ripped from the wall. The soft mode
    /// requires the VM's guest to have VM Tools installed and attempts to
    /// gracefully shut down the VM. Its variant, trySoft, first attempts
    /// a graceful shutdown, and if that fails or the VM is not in a powered off
    /// state after reaching the GuestSoftPowerOffTimeout, the VM is halted.
    /// 
    /// 
    /// If omitted, the mode defaults to hard.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "powerOffMode")]
    pub power_off_mode: Option<VSphereVMPowerOffMode>,
    /// ResourcePool is the name or inventory path of the resource pool in which
    /// the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourcePool")]
    pub resource_pool: Option<String>,
    /// Server is the IP address or FQDN of the vSphere server on which
    /// the virtual machine is created/located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Snapshot is the name of the snapshot from which to create a linked clone.
    /// This field is ignored if LinkedClone is not enabled.
    /// Defaults to the source's current snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    /// StoragePolicyName of the storage policy to use with this
    /// Virtual Machine
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storagePolicyName")]
    pub storage_policy_name: Option<String>,
    /// TagIDs is an optional set of tags to add to an instance. Specified tagIDs
    /// must use URN-notation instead of display names.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tagIDs")]
    pub tag_i_ds: Option<Vec<String>>,
    /// Template is the name or inventory path of the template used to clone
    /// the virtual machine.
    pub template: String,
    /// Thumbprint is the colon-separated SHA-1 checksum of the given vCenter server's host certificate
    /// When this is set to empty, this VirtualMachine would be created
    /// without TLS certificate validation of the communication between Cluster API Provider vSphere
    /// and the VMware vCenter server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}

/// BootstrapRef is a reference to a bootstrap provider-specific resource
/// that holds configuration details.
/// This field is optional in case no bootstrap data is required to create
/// a VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMBootstrapRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// Network is the network configuration for this machine's VM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetwork {
    /// Devices is the list of network devices used by the virtual machine.
    /// TODO(akutz) Make sure at least one network matches the
    ///             ClusterSpec.CloudProviderConfiguration.Network.Name
    pub devices: Vec<VSphereVMNetworkDevices>,
    /// PreferredAPIServeCIDR is the preferred CIDR for the Kubernetes API
    /// server endpoint on this machine
    /// 
    /// 
    /// Deprecated: This field is going to be removed in a future release.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredAPIServerCidr")]
    pub preferred_api_server_cidr: Option<String>,
    /// Routes is a list of optional, static routes applied to the virtual
    /// machine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<VSphereVMNetworkRoutes>>,
}

/// NetworkDeviceSpec defines the network configuration for a virtual machine's
/// network device.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkDevices {
    /// AddressesFromPools is a list of IPAddressPools that should be assigned
    /// to IPAddressClaims. The machine's cloud-init metadata will be populated
    /// with IPAddresses fulfilled by an IPAM provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "addressesFromPools")]
    pub addresses_from_pools: Option<Vec<VSphereVMNetworkDevicesAddressesFromPools>>,
    /// DeviceName may be used to explicitly assign a name to the network device
    /// as it exists in the guest operating system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceName")]
    pub device_name: Option<String>,
    /// DHCP4 is a flag that indicates whether or not to use DHCP for IPv4
    /// on this device.
    /// If true then IPAddrs should not contain any IPv4 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp4: Option<bool>,
    /// DHCP4Overrides allows for the control over several DHCP behaviors.
    /// Overrides will only be applied when the corresponding DHCP flag is set.
    /// Only configured values will be sent, omitted values will default to
    /// distribution defaults.
    /// Dependent on support in the network stack for your distribution.
    /// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcp4Overrides")]
    pub dhcp4_overrides: Option<VSphereVMNetworkDevicesDhcp4Overrides>,
    /// DHCP6 is a flag that indicates whether or not to use DHCP for IPv6
    /// on this device.
    /// If true then IPAddrs should not contain any IPv6 addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp6: Option<bool>,
    /// DHCP6Overrides allows for the control over several DHCP behaviors.
    /// Overrides will only be applied when the corresponding DHCP flag is set.
    /// Only configured values will be sent, omitted values will default to
    /// distribution defaults.
    /// Dependent on support in the network stack for your distribution.
    /// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dhcp6Overrides")]
    pub dhcp6_overrides: Option<VSphereVMNetworkDevicesDhcp6Overrides>,
    /// Gateway4 is the IPv4 gateway used by this device.
    /// Required when DHCP4 is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway4: Option<String>,
    /// Gateway4 is the IPv4 gateway used by this device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway6: Option<String>,
    /// IPAddrs is a list of one or more IPv4 and/or IPv6 addresses to assign
    /// to this device. IP addresses must also specify the segment length in
    /// CIDR notation.
    /// Required when DHCP4, DHCP6 and SkipIPAllocation are false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddrs")]
    pub ip_addrs: Option<Vec<String>>,
    /// MACAddr is the MAC address used by this device.
    /// It is generally a good idea to omit this field and allow a MAC address
    /// to be generated.
    /// Please note that this value must use the VMware OUI to work with the
    /// in-tree vSphere cloud provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "macAddr")]
    pub mac_addr: Option<String>,
    /// MTU is the device’s Maximum Transmission Unit size in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i64>,
    /// Nameservers is a list of IPv4 and/or IPv6 addresses used as DNS
    /// nameservers.
    /// Please note that Linux allows only three nameservers (https://linux.die.net/man/5/resolv.conf).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// NetworkName is the name of the vSphere network to which the device
    /// will be connected.
    #[serde(rename = "networkName")]
    pub network_name: String,
    /// Routes is a list of optional, static routes applied to the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<VSphereVMNetworkDevicesRoutes>>,
    /// SearchDomains is a list of search domains used when resolving IP
    /// addresses with DNS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "searchDomains")]
    pub search_domains: Option<Vec<String>>,
    /// SkipIPAllocation allows the device to not have IP address or DHCP configured.
    /// This is suitable for devices for which IP allocation is handled externally, eg. using Multus CNI.
    /// If true, CAPV will not verify IP address allocation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipIPAllocation")]
    pub skip_ip_allocation: Option<bool>,
}

/// TypedLocalObjectReference contains enough information to let you locate the
/// typed referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkDevicesAddressesFromPools {
    /// APIGroup is the group for the resource being referenced.
    /// If APIGroup is not specified, the specified Kind must be in the core API group.
    /// For any other third-party types, APIGroup is required.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiGroup")]
    pub api_group: Option<String>,
    /// Kind is the type of resource being referenced
    pub kind: String,
    /// Name is the name of resource being referenced
    pub name: String,
}

/// DHCP4Overrides allows for the control over several DHCP behaviors.
/// Overrides will only be applied when the corresponding DHCP flag is set.
/// Only configured values will be sent, omitted values will default to
/// distribution defaults.
/// Dependent on support in the network stack for your distribution.
/// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkDevicesDhcp4Overrides {
    /// Hostname is the name which will be sent to the DHCP server instead of
    /// the machine's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// RouteMetric is used to prioritize routes for devices. A lower metric for
    /// an interface will have a higher priority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeMetric")]
    pub route_metric: Option<i64>,
    /// SendHostname when `true`, the hostname of the machine will be sent to the
    /// DHCP server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendHostname")]
    pub send_hostname: Option<bool>,
    /// UseDNS when `true`, the DNS servers in the DHCP server will be used and
    /// take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDNS")]
    pub use_dns: Option<bool>,
    /// UseDomains can take the values `true`, `false`, or `route`. When `true`,
    /// the domain name from the DHCP server will be used as the DNS search
    /// domain for this device. When `route`, the domain name from the DHCP
    /// response will be used for routing DNS only, not for searching.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDomains")]
    pub use_domains: Option<String>,
    /// UseHostname when `true`, the hostname from the DHCP server will be set
    /// as the transient hostname of the machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useHostname")]
    pub use_hostname: Option<bool>,
    /// UseMTU when `true`, the MTU from the DHCP server will be set as the
    /// MTU of the device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useMTU")]
    pub use_mtu: Option<bool>,
    /// UseNTP when `true`, the NTP servers from the DHCP server will be used
    /// by systemd-timesyncd and take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useNTP")]
    pub use_ntp: Option<bool>,
    /// UseRoutes when `true`, the routes from the DHCP server will be installed
    /// in the routing table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRoutes")]
    pub use_routes: Option<String>,
}

/// DHCP6Overrides allows for the control over several DHCP behaviors.
/// Overrides will only be applied when the corresponding DHCP flag is set.
/// Only configured values will be sent, omitted values will default to
/// distribution defaults.
/// Dependent on support in the network stack for your distribution.
/// For more information see the netplan reference (https://netplan.io/reference#dhcp-overrides)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkDevicesDhcp6Overrides {
    /// Hostname is the name which will be sent to the DHCP server instead of
    /// the machine's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// RouteMetric is used to prioritize routes for devices. A lower metric for
    /// an interface will have a higher priority.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeMetric")]
    pub route_metric: Option<i64>,
    /// SendHostname when `true`, the hostname of the machine will be sent to the
    /// DHCP server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sendHostname")]
    pub send_hostname: Option<bool>,
    /// UseDNS when `true`, the DNS servers in the DHCP server will be used and
    /// take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDNS")]
    pub use_dns: Option<bool>,
    /// UseDomains can take the values `true`, `false`, or `route`. When `true`,
    /// the domain name from the DHCP server will be used as the DNS search
    /// domain for this device. When `route`, the domain name from the DHCP
    /// response will be used for routing DNS only, not for searching.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDomains")]
    pub use_domains: Option<String>,
    /// UseHostname when `true`, the hostname from the DHCP server will be set
    /// as the transient hostname of the machine.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useHostname")]
    pub use_hostname: Option<bool>,
    /// UseMTU when `true`, the MTU from the DHCP server will be set as the
    /// MTU of the device.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useMTU")]
    pub use_mtu: Option<bool>,
    /// UseNTP when `true`, the NTP servers from the DHCP server will be used
    /// by systemd-timesyncd and take precedence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useNTP")]
    pub use_ntp: Option<bool>,
    /// UseRoutes when `true`, the routes from the DHCP server will be installed
    /// in the routing table.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useRoutes")]
    pub use_routes: Option<String>,
}

/// NetworkRouteSpec defines a static network route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkDevicesRoutes {
    /// Metric is the weight/priority of the route.
    pub metric: i32,
    /// To is an IPv4 or IPv6 address.
    pub to: String,
    /// Via is an IPv4 or IPv6 address.
    pub via: String,
}

/// NetworkRouteSpec defines a static network route.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMNetworkRoutes {
    /// Metric is the weight/priority of the route.
    pub metric: i32,
    /// To is an IPv4 or IPv6 address.
    pub to: String,
    /// Via is an IPv4 or IPv6 address.
    pub via: String,
}

/// PCIDeviceSpec defines virtual machine's PCI configuration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMPciDevices {
    /// CustomLabel is the hardware label of a virtual machine's PCI device.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "customLabel")]
    pub custom_label: Option<String>,
    /// DeviceID is the device ID of a virtual machine's PCI, in integer.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with VGPUProfile as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceId")]
    pub device_id: Option<i32>,
    /// VGPUProfile is the profile name of a virtual machine's vGPU, in string.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with DeviceID and VendorID as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vGPUProfile")]
    pub v_gpu_profile: Option<String>,
    /// VendorId is the vendor ID of a virtual machine's PCI, in integer.
    /// Defaults to the eponymous property value in the template from which the
    /// virtual machine is cloned.
    /// Mutually exclusive with VGPUProfile as VGPUProfile and DeviceID + VendorID
    /// are two independent ways to define PCI devices.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vendorId")]
    pub vendor_id: Option<i32>,
}

/// VSphereVMSpec defines the desired state of VSphereVM.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VSphereVMPowerOffMode {
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "soft")]
    Soft,
    #[serde(rename = "trySoft")]
    TrySoft,
}

/// VSphereVMStatus defines the observed state of VSphereVM.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMStatus {
    /// Addresses is a list of the VM's IP addresses.
    /// This field is required at runtime for other controllers that read
    /// this CRD as unstructured data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// CloneMode is the type of clone operation used to clone this VM. Since
    /// LinkedMode is the default but fails gracefully if the source of the
    /// clone has no snapshots, this field may be used to determine the actual
    /// type of clone operation used to create this VM.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneMode")]
    pub clone_mode: Option<String>,
    /// Conditions defines current service state of the VSphereVM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureMessage will be set in the event that there is a terminal problem
    /// reconciling the vspherevm and will contain a more verbose string suitable
    /// for logging and human consumption.
    /// 
    /// 
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the vm.
    /// 
    /// 
    /// Any transient errors that occur during the reconciliation of vspherevms
    /// can be added as events to the vspherevm object and/or logged in the
    /// controller's output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureMessage")]
    pub failure_message: Option<String>,
    /// FailureReason will be set in the event that there is a terminal problem
    /// reconciling the vspherevm and will contain a succinct value suitable
    /// for vm interpretation.
    /// 
    /// 
    /// This field should not be set for transitive errors that a controller
    /// faces that are expected to be fixed automatically over
    /// time (like service outages), but instead indicate that something is
    /// fundamentally wrong with the vm.
    /// 
    /// 
    /// Any transient errors that occur during the reconciliation of vspherevms
    /// can be added as events to the vspherevm object and/or logged in the
    /// controller's output.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureReason")]
    pub failure_reason: Option<String>,
    /// Host describes the hostname or IP address of the infrastructure host
    /// that the VSphereVM is residing on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// ModuleUUID is the unique identifier for the vCenter cluster module construct
    /// which is used to configure anti-affinity. Objects with the same ModuleUUID
    /// will be anti-affined, meaning that the vCenter DRS will best effort schedule
    /// the VMs on separate hosts.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "moduleUUID")]
    pub module_uuid: Option<String>,
    /// Network returns the network status for each of the machine's configured
    /// network interfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<VSphereVMStatusNetwork>>,
    /// Ready is true when the provider resource is ready.
    /// This field is required at runtime for other controllers that read
    /// this CRD as unstructured data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// RetryAfter tracks the time we can retry queueing a task
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryAfter")]
    pub retry_after: Option<String>,
    /// Snapshot is the name of the snapshot from which the VM was cloned if
    /// LinkedMode is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    /// TaskRef is a managed object reference to a Task related to the machine.
    /// This value is set automatically at runtime and should not be set or
    /// modified by users.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "taskRef")]
    pub task_ref: Option<String>,
    /// VMRef is the VM's Managed Object Reference on vSphere. It can be used by consumers
    /// to programatically get this VM representation on vSphere in case of the need to retrieve informations.
    /// This field is set once the machine is created and should not be changed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vmRef")]
    pub vm_ref: Option<String>,
}

/// NetworkStatus provides information about one of a VM's networks.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VSphereVMStatusNetwork {
    /// Connected is a flag that indicates whether this network is currently
    /// connected to the VM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// IPAddrs is one or more IP addresses reported by vm-tools.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipAddrs")]
    pub ip_addrs: Option<Vec<String>>,
    /// MACAddr is the MAC address of the network device.
    #[serde(rename = "macAddr")]
    pub mac_addr: String,
    /// NetworkName is the name of the network.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkName")]
    pub network_name: Option<String>,
}

