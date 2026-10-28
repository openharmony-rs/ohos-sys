# Changelog

## 0.1.0

- Initial release. Bindings to `libohresmgr.so` (resourcemanager) for API-12 through API-26.
  Covers obtaining strings, string arrays, colors, numbers, booleans, media, drawable
  descriptors, symbols, locales and the device configuration by resource ID or name, adding
  and removing overlay resources, plural strings
  (`OH_ResourceManager_GetIntPluralString`, `OH_ResourceManager_GetDoublePluralString` and
  their `ByName` variants, API-18) and `OH_ResourceManager_GetResourceConfiguration` (API-20).
