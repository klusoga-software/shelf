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
2. Make sure you created a public client inside you oidc server for the shelf ui. Please follow the instructions here
   for an example with keycloak [](Keycloak.md)

3. Please update the helm values for your needs:

<code-block lang="yaml" src="helm-values.yaml" collapsible="true" collapsed-title="Default helm values"/>

4. Deploy with helm:
<code-block lang="shell">helm install shelf</code-block>

