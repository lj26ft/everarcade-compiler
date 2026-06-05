export function localDeployment(project) { return { kind: 'localDeployment', project, target: 'local' }; }
export function leaseDeployment(project, leaseId) { return { kind: 'leaseDeployment', project, leaseId, target: 'evernode-lease' }; }
export function federationDeployment(project, federationId) { return { kind: 'federationDeployment', project, federationId, target: 'federation' }; }
