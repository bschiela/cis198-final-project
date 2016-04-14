//! Defines the X-Amz-Target and X-Amz-Date custom HTTP headers using hyper macros

header! { (XAmzTarget, "X-Amz-Target") => [String] }
header! { (XAmzDate, "X-Amz-Date") => [String] }
