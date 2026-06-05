export function run() {
  return {
    name: 'Arena Example',
    loop: 'build-deploy-run',
    protocolKnowledgeRequired: false
  };
}

console.log(JSON.stringify(run(), null, 2));
