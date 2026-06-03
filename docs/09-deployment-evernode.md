# 09. Deployment and Evernode

## Purpose

Deployment moves certified runtime artifacts to target hosts and operates them. Evernode is the platform deployment target for hosted runtime nodes.

## Current Status

Evernode deployment is partial. Provider code, templates, reports, manifests, and validation artifacts exist, but production commercial hosting requires completed release gates, automation, rollback, monitoring, operator policy, and certification.

## Deployment Responsibilities

- install runtime artifacts;
- configure runtime roots, packages, and environment;
- verify artifact hashes and manifests;
- start, stop, upgrade, roll back, and recover nodes;
- collect health and operational evidence;
- preserve release certification evidence.

## Deployment Non-Responsibilities

Deployment does not decide canonical state, certify game logic by itself, or replace runtime replay verification.

## Evernode Production Gates

Commercial Evernode deployment requires offline build reproducibility, signed artifacts, deployment manifest verification, capacity policy, storage policy, observability, backup/restore drills, incident response, billing boundaries, and operator onboarding.
