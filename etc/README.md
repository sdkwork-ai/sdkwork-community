# Source Configuration

`sdkwork.deployment.config.json` is the source profile index for the SDKWork Community application.
It selects safe, source-controlled profiles under `topology/`; `specs/topology.spec.json` defines the
allowed topology and `sdkwork.app.config.json` defines application and release identity.

Supported profiles are `standalone.development`, `standalone.production`, `cloud.development`, and
`cloud.production`. Local development defaults to `standalone.development`. Profile values are
materialized by `sdkwork-app`; generated runtime files and local overrides are not source authority.

Files matching `*.local.*`, `.sdkwork/local/`, and `.sdkwork/secrets/` are private and ignored. Keep
passwords, tokens, keys, and certificates out of this directory. PostgreSQL and Redis credentials
must be provided through referenced local files, mounted secret files, or protected process
environment values. `standalone.production` requires PostgreSQL and Redis before serving traffic.
`cloud.production` intentionally omits managed database and Redis hosts; the platform runtime must
inject the approved host or advanced URL, and missing values fail closed.

Validate after any change:

```powershell
node ..\sdkwork-specs\tools\check-source-config-standard.mjs --root . --enforce-profile-identity
pnpm topology:validate
pnpm deploy:validate
```

<!-- SDKWORK-DEPLOY-LAYOUT: v1 -->
## Installed Runtime Paths

Authority: `APPLICATION_DEPLOY_LAYOUT_SPEC.md` (`../sdkwork-specs/`).

| Item | Value |
| --- | --- |
| `appId` | `sdkwork-community` |
| `runtimeCode` | `community` |
| Config root | `/etc/sdkwork/community/` |
| Runtime TOML | `/etc/sdkwork/community/config.toml` |
| Secrets | `/etc/sdkwork/community/secrets/` |
| Override | `SDKWORK_COMMUNITY_CONFIG_FILE` |

Source profiles live under `etc/` (`sdkwork.deployment.config.json` index). Deploy manifest: `deployments/deploy.yaml`. Web data-plane source: `deployments/webserver/` (`SDKWORK_WEBSERVER_SPEC.md` layout v3).

```bash
node ../sdkwork-specs/tools/check-source-config-standard.mjs --root .
node ../sdkwork-specs/tools/check-application-deploy-layout.mjs --root .
node ../sdkwork-specs/tools/check-webserver-toml-standard.mjs --root deployments/webserver
```
<!-- /SDKWORK-DEPLOY-LAYOUT -->


