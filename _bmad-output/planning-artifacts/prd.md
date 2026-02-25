---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-02b-vision
  - step-02c-executive-summary
  - step-03-success
inputDocuments:
  - source: oral-briefing
    description: "Briefing Robin - pivot scope, Helm catalog, cible non-technique"
  - source: /Users/robinstraub/Projects/robinstraub/controlplane
    description: "Nouveau controlplane (brouillon) - patterns billing Stripe, CaaS K8s, Fabrique ORM"
  - source: /Users/robinstraub/Projects/france-nuage/plateforme
    description: "Ancien projet plateforme - architecture complète (gRPC, React/Chakra/Redux, SpiceDB, Keycloak)"
  - source: https://france-nuage.fr
    description: "Site vitrine - positionnement, offres, documentation publique"
documentCounts:
  briefs: 0
  research: 0
  brainstorming: 0
  projectDocs: 2
workflowType: prd
projectType: brownfield-pivot
classification:
  projectType: saas_b2b
  domain: cloud-paas-souverain
  complexity: medium
  projectContext: brownfield-pivot
---

# Product Requirements Document - france-nuage

**Author:** Robin
**Date:** 2026-02-25

## Executive Summary

France-nuage est une plateforme SaaS souveraine permettant aux TPE, PME et ETI françaises de déployer l'intégralité de leur système d'information depuis une console web unique, sans compétences techniques. L'utilisateur sélectionne des applications pré-packagées (outils collaboratifs, CRM, analytics, stockage...) dans un catalogue, configure quelques paramètres via un formulaire, et l'application est provisionnée automatiquement sur une infrastructure Kubernetes hébergée en France.

Le control plane, écrit en Rust avec une API gRPC, agit comme un passe-plat entre la console web et le cluster Kubernetes. Il gère le provisionnement des applications via des Helm charts façonnés par l'équipe DevOps, l'authentification via Keycloak, l'autorisation fine via SpiceDB (ReBAC), et la facturation mensuelle via Stripe.

Ce projet est un pivot stratégique : l'ancien France-nuage visait un cloud complet depuis les couches matérielles. La nouvelle approche commence par les couches hautes (applications/conteneurs) pour atteindre la rentabilité rapidement, avec l'objectif long terme de redescendre progressivement vers l'infrastructure physique.

### Ce qui rend ce produit spécial

Le marché des alternatives souveraines aux GAFAM est fragmenté : un dirigeant qui veut quitter Google Workspace doit assembler lui-même Nextcloud, Odoo, Metabase, chacun chez un hébergeur différent. France-nuage unifie cette expérience en un point d'entrée unique, centralisé, français, et réinternalisable à 100%. Le moment décisif pour l'utilisateur : découvrir qu'il peut disposer de tout son SI souverain en quelques clics, sans ouvrir un terminal.

Le timing est dicté par le contexte géopolitique : tensions économiques avec les États-Unis, prise de conscience du Cloud Act, et demande croissante des dirigeants pour des alternatives à Microsoft et Google.

## Project Classification

- **Type :** SaaS B2B (plateforme multi-tenant, catalogue d'applications, billing par abonnement)
- **Domaine :** Cloud PaaS souverain français
- **Complexité :** Moyenne (scope resserré au passe-plat K8s + billing, pas de gestion d'infrastructure)
- **Contexte :** Brownfield-pivot (nouvelle base de code, patterns éprouvés réutilisés depuis l'ancien projet)

## Success Criteria

### User Success

- Un utilisateur non-technique déploie sa première application en moins de 15 minutes après inscription
- Le catalogue propose 10 applications couvrant les besoins SI essentiels (collaboratif, stockage, CRM, analytics...)
- Chaque application du catalogue est sélectionnée pour offrir une expérience comparable aux alternatives GAFAM (curation rigoureuse : OnlyOffice plutôt que Collabora, pas de Nextcloud)
- L'utilisateur gère l'ensemble de son SI depuis une console unique sans ouvrir un terminal

### Business Success

- **Base actuelle :** 1 500 EUR/mois de CA
- **Objectif 12 mois (fin 2026) :** 10 000 EUR/mois de CA
- Conversion des clients existants (qui utilisent déjà des apps hébergées) vers la console web en self-service
- Acquisition de nouveaux clients TPE/PME/ETI via le positionnement souverain

### Technical Success

- 100% code coverage sur le control plane Rust
- Architecture extensible par des agents IA (modules découplés, conventions claires)
- Codegen gRPC propre : un .proto modifié = types TypeScript régénérés automatiquement
- SLA et performance : délégués à l'équipe DevOps, pas de cible dans le control plane

### Measurable Outcomes

- Temps de provisionnement d'une app : fonctionnel (pas de cible de performance imposée au MVP)
- Nombre d'applications au catalogue : 10 au lancement
- Taux de conversion clients existants -> console : à mesurer post-lancement

## Product Scope

### MVP - Minimum Viable Product

- Console web (React/Chakra/Redux) avec authentification Keycloak
- Catalogue de 10 applications pré-packagées (Helm charts)
- Formulaire dynamique généré depuis les variables Helm pour chaque application
- Provisionnement d'une application sur K8s via le control plane
- Billing Stripe : 1 app = 1 produit = 1 abonnement mensuel
- Autorisation ReBAC via SpiceDB (organisations, projets)
- Gestion basique des organisations et utilisateurs

### Growth Features (Post-MVP)

- Enrichissement du catalogue d'applications
- Monitoring/statut des applications déployées depuis la console
- Gestion avancée des organisations (invitations, rôles, service accounts)
- Domaines personnalisés pour les applications déployées
- Tableau de bord facturation et historique

### Vision (Future)

- Redescente vers les couches basses : infrastructure physique, VM, stockage
- Cloud 100% français, 100% réinternalisable depuis le matériel
- Marketplace ouverte de Helm charts (contributions tierces)
