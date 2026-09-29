# Changelog

## Unreleased

- Update bindings to API-24, API-25 and API-26 (no new symbols).

## 0.1.0

- Initial release. Bindings to `libqos.so` for API-12 through API-23.
  Covers the QoS thread scheduling APIs (`OH_QoS_SetThreadQoS`,
  `OH_QoS_ResetThreadQoS`, `OH_QoS_GetThreadQoS`) and the Gewu on-device
  LLM inference APIs (`OH_QoS_GewuCreateSession`,
  `OH_QoS_GewuSubmitRequest`, …) introduced at API-20.
