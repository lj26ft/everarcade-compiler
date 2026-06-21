const DEFAULT_PROTOCOL_VERSION = 'adapter-sdk-v1';

function normalizeEndpoint(endpoint) {
  if (!endpoint) {
    throw new Error('connectWorld requires an endpoint');
  }
  return endpoint.replace(/\/$/, '');
}

async function requestJson(url, options = {}) {
  const response = await fetch(url, {
    ...options,
    headers: {
      'content-type': 'application/json',
      ...(options.headers || {}),
    },
  });

  if (!response.ok) {
    const text = await response.text();
    throw new Error(`EverArcade adapter request failed: ${response.status} ${text}`);
  }

  if (response.status === 204) {
    return null;
  }

  return response.json();
}

export async function connectWorld(config) {
  const endpoint = normalizeEndpoint(config?.endpoint);
  const body = {
    protocolVersion: config.protocolVersion || DEFAULT_PROTOCOL_VERSION,
    worldId: config.worldId,
    playerId: config.playerId,
    client: config.client || 'browser',
  };

  const session = await requestJson(`${endpoint}/world/connect`, {
    method: 'POST',
    body: JSON.stringify(body),
    headers: config.headers,
  });

  return {
    endpoint,
    headers: config.headers || {},
    sessionId: session.sessionId,
    worldId: session.worldId || config.worldId,
    protocolVersion: session.protocolVersion || body.protocolVersion,
  };
}

export async function submitInput(connection, input) {
  return requestJson(`${connection.endpoint}/world/${connection.worldId}/input`, {
    method: 'POST',
    body: JSON.stringify({ sessionId: connection.sessionId, input }),
    headers: connection.headers,
  });
}

export async function readProjection(connection, query = {}) {
  const params = new URLSearchParams({ sessionId: connection.sessionId });
  for (const [key, value] of Object.entries(query)) {
    if (value !== undefined && value !== null) {
      params.set(key, String(value));
    }
  }

  return requestJson(`${connection.endpoint}/world/${connection.worldId}/projection?${params}`, {
    headers: connection.headers,
  });
}

export async function verifyWorld(connection, proof = {}) {
  return requestJson(`${connection.endpoint}/world/${connection.worldId}/verify`, {
    method: 'POST',
    body: JSON.stringify({ sessionId: connection.sessionId, proof }),
    headers: connection.headers,
  });
}

export async function disconnect(connection) {
  await requestJson(`${connection.endpoint}/world/disconnect`, {
    method: 'POST',
    body: JSON.stringify({ sessionId: connection.sessionId, worldId: connection.worldId }),
    headers: connection.headers,
  });
}
