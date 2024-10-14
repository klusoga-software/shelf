<secondary-label ref="wip"/>

# Deployment

## Requirements

Shelf does need the following requirements to be deployed:

- A PostgreSQL Database
- An OIDC Provider like Keycloak

## Deploy with Helm

<warning>
When updating to a new major version of shelf, make sure to backup your database
</warning>

1. Create a new database and user for shelf. Shelf only needs one database with a owner for that database
2. Make sure you created a public client inside you oidc server for the shelf ui. There are no roles or special scopes needed yet.

3. Please update the helm values for your needs:

<code-block lang="yaml" src="helm-values.yaml" collapsible="true" collapsed-title="Default helm values"/>

4. Deploy with helm:
<code-block lang="shell">helm install shelf oci://ghcr.io/klusoga-software --version 0.2.0 --set-file values.yaml</code-block>

