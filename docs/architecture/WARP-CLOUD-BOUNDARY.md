# Warp Cloud / Local AI Boundary

## Contract

Local terminal startup and local AI inference must work without Warp cloud identity, subscription, entitlement, quota, or hosted inference. Local AI may use generic HTTP, the existing secure-storage mechanism, provider limits, and model limits.

Cloud collaboration, sync, workspace state, artifacts, team features, and unrelated GraphQL APIs remain on the existing cloud stack.

## Classification

| Area | Category | Local AI dependency | Replacement / disposition |
| --- | --- | --- | --- |
| Warp login and account session | AUTH / CLOUD_FEATURE | No | Keep for cloud-only features; never gate local startup or provider setup |
| Warp AI hosted generation | LOCAL_AI | No | Provider-neutral `InferenceProvider` and `InferenceRouter` |
| Warp subscriptions, entitlements, credits, quotas | ENTITLEMENT | No | Remove from local inference; preserve provider billing/rate limits |
| Model context and output limits | MODEL_LIMIT | Yes | Normalize provider errors and preserve model constraints |
| Provider rate limits and spending limits | PROVIDER_LIMIT | Yes | Normalize errors without translating them to Warp quota errors |
| Conversation persistence and workspace state | CLOUD_FEATURE | Optional | Keep existing cloud persistence where the feature requires it |
| Provider credentials | AUTH | Yes, through secure storage only | Namespaced `ai/<provider>/api_key` credentials |

## Forbidden local-AI dependencies

The following may not be referenced from `app/src/ai`, `app/src/ai_assistant`, `app/src/terminal`, or `crates/ai`: `warp_server_auth`, `generate_multi_agent_output`, `QuotaLimit`, `WarpSubscription`, and `WarpEntitlement`.

This is intentionally scoped. Cloud-only code elsewhere in the repository may continue to use existing account and server clients until their call sites are independently migrated.

## Migration boundary

`Agent API -> InferenceRouter -> InferenceProvider -> HTTP` is the target generation path. `CloudAccount` and cloud GraphQL remain a separate branch for collaboration, sync, workspace, artifacts, team, and account features.

Provider keys must never be serialized into ordinary settings, telemetry, crash payloads, or debug output, and cloud logout/configuration must not mutate provider credentials.

## Removal order

1. Add provider contracts and boundary checks.
2. Implement direct OpenAI-compatible inference and provider adapters.
3. Migrate local agent generation and remove local product gates.
4. Verify startup and cloud feature preservation.
5. Remove redundant server operations and delete auth/client crates only if no required cloud call site remains.

## Inventory command

```bash
rg -n -i 'warp\\.dev|graphql|warp_server_auth|warp_server_client|generate_multi_agent_output|subscription|entitlement|quota|credit|upgrade|token.?limit' app crates
```

Every match must be classified as `ENTITLEMENT`, `PROVIDER_LIMIT`, `MODEL_LIMIT`, `RETRY`, `TELEMETRY`, `UI`, `CLOUD_FEATURE`, `LOCAL_AI`, `AUTH`, or `UNKNOWN` before removal.

## Acceptance criteria

- Local AI has no Warp auth or hosted-generation dependency.
- Missing provider configuration is an AI error, not a startup failure.
- OpenRouter, Kilo, and custom OpenAI-compatible providers share provider-neutral contracts.
- Existing cloud functionality remains available where required.
- The scoped boundary script stays green.
