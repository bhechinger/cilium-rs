// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium ciliumloadbalancerippools.cilium.io -A
// kopium version: 0.16.2

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Spec is a human readable description for a BGP load balancer ip pool.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "cilium.io", version = "v2alpha1", kind = "CiliumLoadBalancerIPPool", plural = "ciliumloadbalancerippools")]
#[kube(status = "CiliumLoadBalancerIPPoolStatus")]
pub struct CiliumLoadBalancerIPPoolSpec {
    /// CiliumLoadBalancerIPPoolCIDRBlock is a list of CIDRs comprising this IP Pool
    pub cidrs: Vec<CiliumLoadBalancerIPPoolCidrs>,
    /// Disabled, if set to true means that no new IPs will be allocated from this pool. Existing allocations will not be removed from services.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// ServiceSelector selects a set of services which are eligible to receive IPs from this
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceSelector")]
    pub service_selector: Option<CiliumLoadBalancerIPPoolServiceSelector>,
}

/// CiliumLoadBalancerIPPoolCIDRBlock describes a single CIDR block.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CiliumLoadBalancerIPPoolCidrs {
    pub cidr: String,
}

/// ServiceSelector selects a set of services which are eligible to receive IPs from this
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CiliumLoadBalancerIPPoolServiceSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CiliumLoadBalancerIPPoolServiceSelectorMatchExpressions>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CiliumLoadBalancerIPPoolServiceSelectorMatchExpressions {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: CiliumLoadBalancerIPPoolServiceSelectorMatchExpressionsOperator,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum CiliumLoadBalancerIPPoolServiceSelectorMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
}

/// Status is the status of the IP Pool. 
///  It might be possible for users to define overlapping IP Pools, we can't validate or enforce non-overlapping pools during object creation. The Cilium operator will do this validation and update the status to reflect the ability to allocate IPs from this pool.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CiliumLoadBalancerIPPoolStatus {
    /// Current service state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CiliumLoadBalancerIPPoolStatusConditions>>,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  	type FooStatus struct{ 	    // Represents the observations of a foo's current state. 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" 	    // +patchMergeKey=type 	    // +patchStrategy=merge 	    // +listType=map 	    // +listMapKey=type 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  	    // other fields 	}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CiliumLoadBalancerIPPoolStatusConditions {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    pub message: String,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions[x].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    pub reason: String,
    /// status of the condition, one of True, False, Unknown.
    pub status: CiliumLoadBalancerIPPoolStatusConditionsStatus,
    /// type of condition in CamelCase or in foo.example.com/CamelCase. --- Many .condition.type values are consistent across resources like Available, but because arbitrary conditions can be useful (see .node.status.conditions), the ability to deconflict is important. The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Condition contains details for one aspect of the current state of this API Resource. --- This struct is intended for direct use as an array at the field path .status.conditions.  For example, 
///  	type FooStatus struct{ 	    // Represents the observations of a foo's current state. 	    // Known .status.conditions.type are: "Available", "Progressing", and "Degraded" 	    // +patchMergeKey=type 	    // +patchStrategy=merge 	    // +listType=map 	    // +listMapKey=type 	    Conditions []metav1.Condition `json:"conditions,omitempty" patchStrategy:"merge" patchMergeKey:"type" protobuf:"bytes,1,rep,name=conditions"` 
///  	    // other fields 	}
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum CiliumLoadBalancerIPPoolStatusConditionsStatus {
    True,
    False,
    Unknown,
}

