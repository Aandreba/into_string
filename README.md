`into_string` is a helper crate to convert values into strings avoiding extra allocations.

> **Note**\
> Some of the functionality of this crate must be enabled with the `max` feature.
> When `max` is disabled, `into_string` uses the `min_specialization` API, which lacks some of the features of `specialization`, but is complete (currently, `specialization` is marked as an incomplete feature and shows a warning when enabled) 
