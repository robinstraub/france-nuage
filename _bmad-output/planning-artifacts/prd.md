---
stepsCompleted:
  - step-01-init
  - step-02-discovery
  - step-02b-vision
  - step-02c-executive-summary
  - step-03-success
  - step-04-journeys
  - step-05-domain
  - step-06-innovation
  - step-07-project-type
  - step-08-scoping
  - step-09-functional
  - step-10-nonfunctional
  - step-11-polish
  - step-12-complete
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

## User Journeys

### Parcours 1 : Sophie, dirigeante de TPE - Découverte et premier déploiement

Sophie dirige un cabinet de conseil de 12 personnes. Depuis les annonces sur les tarifs cloud américains et les tensions géopolitiques, elle cherche à sortir de Google Workspace. Elle a entendu parler de Nextcloud, OnlyOffice, mais ne sait pas par où commencer ni comment héberger tout ça.

**Scène d'ouverture :** Sophie tombe sur France-nuage. Elle s'inscrit, crée son organisation "Cabinet Sophie Martin". L'authentification Keycloak lui donne accès à la console. Elle crée un premier projet "Outils collaboratifs".

**Action montante :** Elle découvre le catalogue d'applications. Elle voit "Suite bureautique - OnlyOffice", "Stockage de fichiers", "CRM". Chaque application a une description claire de ce qu'elle fait et de son équivalent GAFAM. Elle sélectionne OnlyOffice. Un formulaire lui demande quelques informations simples : nom de son instance, email de contact. Pas de jargon technique.

**Moment décisif :** Elle clique "Déployer". En quelques instants, son OnlyOffice est provisionné. Elle reçoit une URL d'accès. Elle ouvre le lien : ça fonctionne. Son abonnement Stripe est actif. Elle se dit : "C'est tout ? C'est aussi simple que ça ?"

**Résolution :** Au fil des semaines, Sophie ajoute un outil de stockage, puis un CRM. Tout est dans sa console, centralisé, hébergé en France. Elle sait exactement ce qu'elle paie. Ses collaborateurs utilisent les apps sans même savoir que ce n'est pas Google.

**Scénario d'erreur :** Sophie se trompe dans un champ de configuration. Le formulaire lui indique clairement l'erreur avant de soumettre. Si le déploiement échoue (problème K8s), elle voit un statut clair dans sa console et peut réessayer ou contacter le support par email.

### Parcours 2 : Sophie gère son organisation au quotidien

Sophie a maintenant 4 applications déployées. Elle veut voir ce qu'elle paie et éventuellement supprimer une application qu'elle n'utilise plus.

**Scène d'ouverture :** Sophie se connecte à la console et voit son tableau de bord : la liste de ses applications déployées avec leur statut.

**Action montante :** Elle identifie le CRM qu'elle n'utilise finalement pas. Elle clique pour le supprimer. Une confirmation lui demande si elle est sûre. L'abonnement Stripe correspondant est annulé.

**Résolution :** Sophie a une vision claire de son SI, de ce qu'elle paie, et peut ajouter/retirer des briques à volonté.

### Parcours 3 : Robin, admin France-nuage - Gestion via l'organisation racine

Robin est admin de l'organisation racine "France-nuage", parente de toutes les organisations clientes. Il accède à la même console que les clients, mais l'organisation racine lui donne une visibilité sur les organisations enfants grâce au mécanisme ReBAC de SpiceDB (permission héritée via la relation parent).

**Scène d'ouverture :** Un client signale un problème par email. Robin se connecte à la console, navigue vers l'organisation du client via la hiérarchie parent-enfant.

**Action montante :** Il voit les applications déployées du client, leur statut, les informations de configuration. Il peut diagnostiquer le problème sans accès SSH ni outil séparé.

**Résolution :** Robin identifie le problème et peut intervenir si nécessaire. À terme, des interfaces d'administration dédiées seront ajoutées, mais le mécanisme d'organisation parent-enfant couvre les besoins MVP.

### Parcours 4 : Équipe DevOps - Déclaration d'un Helm chart (hors console)

L'équipe DevOps a packagé une nouvelle application dans un Helm chart. Pour la rendre disponible dans le catalogue, ils insèrent directement en base de données les métadonnées : nom de l'application, référence du chart, variables configurables (nom, type, description, valeurs par défaut, contraintes de validation).

Ce parcours n'a pas d'interface web au MVP - c'est un insert SQL ou un script. Le control plane et la console prennent le relais automatiquement : le nouveau chart apparaît dans le catalogue, le formulaire dynamique est généré depuis les variables déclarées.

### Journey Requirements Summary

| Parcours | Capacités révélées |
|---|---|
| Sophie - Premier déploiement | Inscription/auth Keycloak, création projet, catalogue d'apps, formulaire dynamique Helm, provisionnement K8s, billing Stripe, feedback de statut |
| Sophie - Gestion quotidienne | Dashboard apps déployées, suppression d'app, annulation abonnement Stripe, vision facturation |
| Admin France-nuage | Hiérarchie organisations parent-enfant (ReBAC SpiceDB), navigation inter-organisations, visibilité apps clients |
| DevOps - Helm chart | Modèle de données catalogue (chart, variables, validation), pas d'interface web MVP |

## Domain-Specific Requirements

### Conformité & réglementaire

- RGPD natif : données hébergées en France, pas de transfert hors UE
- Pas de certification SecNumCloud ou ISO 27001 visée au MVP (objectif long terme)

### Contraintes techniques

- Isolation des tenants sur K8s (namespaces, network policies - responsabilité DevOps)
- Le control plane ne stocke que des identifiants de référence Stripe, pas de secrets ni de données de paiement - les secrets K8s et clés API sont gérés par les DevOps
- Sécurité de l'API gRPC : authentification JWT Keycloak obligatoire sur tous les endpoints

### Risques domaine

- Dépendance à la qualité des Helm charts DevOps (un chart mal configuré = mauvaise expérience client)
- Stripe comme single point of failure billing (acceptable au MVP)

## SaaS B2B Specific Requirements

### Project-Type Overview

France-nuage est un SaaS B2B multi-tenant à facturation mensuelle par organisation. L'unité de valeur est l'application déployée depuis un catalogue. Le modèle est simple par design : pas de tiers complexes, pas d'usage-based, pas de marketplace.

### Tenant Model

- **Organisation** comme unité de tenant, avec hiérarchie parent-enfant
- Organisation racine "France-nuage" parente de toutes les organisations clientes
- Chaque organisation a ses propres projets, applications déployées, abonnements
- Isolation logique via SpiceDB (permissions) et K8s (namespaces, responsabilité DevOps)
- Un utilisateur appartient à une organisation (table `organization_user`)

### Permission Model (ReBAC via SpiceDB)

- Schéma hiérarchique : `organization -> project -> application`
- Permissions héritées via relations parent (l'admin France-nuage voit tout via l'organisation racine)
- MVP : pas de rôles granulaires, tout membre d'une organisation a accès à tout dans cette organisation
- Post-MVP : rôles fins (viewer, editor, admin) supportés par le schéma SpiceDB

### Subscription Model

- 1 organisation = 1 abonnement Stripe (subscription)
- Chaque application déployée = 1 item (produit + prix) dans cet abonnement
- Le client voit un seul prélèvement mensuel regroupant toutes ses apps
- Suppression d'une application = retrait de l'item correspondant de la subscription
- Mois commencé est dû

### Integration Requirements

| Système | Rôle | Protocole |
|---|---|---|
| Keycloak | Authentification (OIDC) | JWT / OIDC Discovery |
| SpiceDB | Autorisation (ReBAC) | gRPC |
| Stripe | Billing (abonnements) | API REST Stripe |
| Kubernetes | Provisionnement apps | kube-rs / Helm |
| PostgreSQL | Persistance | sqlx / Fabrique ORM |

### Implementation Considerations

- **Codegen cross-stack** : les .proto définissent le contrat API, codegen Rust (tonic/prost) et TypeScript (@protobuf-ts) garantissent la cohérence
- **Tests frontend** : contre une vraie instance du control plane (pas de mocks in-memory)
- **Gestion des utilisateurs** : à arbitrer lors de l'architecture - le control plane pourrait interroger Keycloak directement plutôt que de maintenir une copie locale des users. L'ancienne approche (synchronizer + tables users/service_accounts locales) était potentiellement overkill. Point ouvert.
- **gRPC Reflection** pour le discovery d'API
- **100% code coverage Rust** : architecture modulaire avec traits pour faciliter le mocking

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**Approche :** MVP revenue-first. Le marché est validé (clients existants à 1 500 EUR/mois). L'objectif est de livrer un produit fonctionnel en 1 mois pour convertir les clients existants et en acquérir de nouveaux.

**Ressources :** Développeur solo (Robin) assisté par des agents IA pour la génération de code. La méthodologie BMAD sert de cadre pour produire des specs suffisamment précises pour guider les agents et limiter la dérive architecturale.

### MVP Feature Set (Phase 1) - Cible : 1 mois

**Parcours supportés :**
- Sophie déploie sa première application (parcours 1)
- Sophie gère ses applications au quotidien (parcours 2)
- Admin France-nuage via organisation racine (parcours 3)
- DevOps déclare les Helm charts en BDD (parcours 4, hors console)

**Must-Have :**
- Authentification Keycloak (OIDC)
- Gestion organisations (hiérarchie parent-enfant interne)
- Gestion projets (création, consultation, contexte de déploiement)
- Catalogue d'applications (Helm charts déclarés en BDD)
- Formulaire dynamique généré depuis les variables Helm
- Provisionnement d'une application sur K8s via le control plane
- Billing Stripe : 1 subscription par organisation, items par app
- Autorisation ReBAC via SpiceDB
- Console web (React/Chakra/Redux) fonctionnelle

### Post-MVP Features (Phase 2)

- Monitoring/statut des applications déployées
- Gestion avancée des organisations (invitations, rôles)
- Enrichissement du catalogue (> 10 apps)
- Interface d'administration pour les DevOps (déclaration Helm charts)
- Tableau de bord facturation
- CLI et/ou API publique

### Vision (Phase 3)

- Rôles granulaires SpiceDB (viewer, editor, admin)
- Domaines personnalisés
- Redescente vers les couches basses (infra physique, VM, stockage)
- Marketplace Helm charts (contributions tierces)

### Risk Mitigation Strategy

**Risque principal : dérive architecturale via agents IA**
- Mitigation : specs BMAD précises (PRD, architecture) comme guide pour les agents
- 100% code coverage Rust pour détecter les régressions
- Architecture modulaire avec traits (facilite le review et le mocking)
- Conventions de code strictes documentées
- Review humain systématique du code généré

**Risque secondaire : timeline agressive (1 mois)**
- Mitigation : scope MVP minimal et non-négociable
- Réutilisation des patterns éprouvés de l'ancien projet
- Pas de features "nice-to-have" au MVP
- DevOps gèrent les Helm charts directement en BDD (pas d'interface)

**Risque tertiaire : adoption clients existants**
- Mitigation : les clients utilisent déjà les apps hébergées, la console est un nouveau point d'entrée
- Onboarding simple, pas de migration technique pour le client

## Functional Requirements

### Authentification & Identité

- FR1 : Un utilisateur peut s'inscrire et se connecter via Keycloak (OIDC), y compris via fédération d'identité Google
- FR2 : Un utilisateur authentifié peut se déconnecter de la console
- FR3 : Le control plane peut valider un token d'authentification Keycloak sur chaque requête API
- FR4 : Le control plane peut identifier l'utilisateur et ses organisations à partir de son token

### Gestion des organisations

- FR5 : Un utilisateur peut créer une organisation
- FR6 : Un utilisateur peut consulter les organisations dont il est membre
- FR7 : La hiérarchie parent-enfant des organisations est gérée en interne par l'équipe France-nuage (non exposée aux utilisateurs clients)
- FR8 : Un admin de l'organisation racine peut consulter les organisations enfants, leurs projets et leurs applications déployées
- FR9 : L'organisation racine "France-nuage" est parente de toutes les organisations clientes

### Gestion des projets

- FR10 : Un utilisateur peut créer un projet au sein de son organisation
- FR11 : Un utilisateur peut consulter les projets de son organisation
- FR12 : Un utilisateur déploie des applications dans le contexte d'un projet

### Catalogue d'applications

- FR13 : Un utilisateur peut consulter le catalogue des applications disponibles
- FR14 : Un utilisateur peut voir le nom, la description et l'équivalent GAFAM de chaque application du catalogue
- FR15 : Le catalogue est alimenté par des métadonnées Helm charts stockées dans le système
- FR16 : Un utilisateur voit pour chaque application les variables configurables (nom, type, description, valeur par défaut, contraintes de validation)
- FR17 : Un opérateur DevOps peut déclarer un nouveau Helm chart et ses variables directement dans le système

### Provisionnement d'applications

- FR18 : Un utilisateur peut déployer une application depuis le catalogue dans un de ses projets
- FR19 : Un utilisateur voit un formulaire dynamique généré à partir des variables configurables du Helm chart sélectionné
- FR20 : Le formulaire valide les saisies utilisateur selon les contraintes définies avant soumission
- FR21 : Le control plane peut appliquer un Helm chart sur Kubernetes avec les valeurs saisies par l'utilisateur
- FR22 : L'utilisateur reçoit un feedback de statut sur le provisionnement (en cours, succès, échec)
- FR23 : L'utilisateur peut consulter l'URL d'accès de son application une fois déployée

### Gestion des applications déployées

- FR24 : Un utilisateur peut consulter la liste de ses applications déployées avec leur statut
- FR25 : Un utilisateur peut supprimer une application déployée (avec confirmation)
- FR26 : La suppression d'une application déclenche le retrait du Helm release sur K8s

### Billing & Facturation

- FR27 : Le premier déploiement d'une application déclenche la création d'un customer Stripe et la saisie des informations de paiement
- FR28 : Le déploiement d'une application crée une subscription Stripe (si première app) ou ajoute un item à la subscription existante
- FR29 : La suppression d'une application retire l'item correspondant de la subscription Stripe
- FR30 : Un utilisateur peut consulter la liste de ses applications et leur coût mensuel associé

### Autorisation (ReBAC)

- FR31 : Le control plane vérifie les permissions SpiceDB avant toute opération sur une ressource
- FR32 : Un admin d'une organisation parente hérite automatiquement des permissions sur les organisations enfants
- FR33 : Au MVP, tout membre d'une organisation a accès complet aux ressources de cette organisation

## Non-Functional Requirements

### Sécurité

- Toutes les communications API sont authentifiées (authentification Keycloak obligatoire)
- Le control plane ne stocke que des identifiants de référence Stripe, pas de secrets ni de données de paiement
- Aucun secret n'est exposé dans les réponses API ou les logs
- Les données utilisateur restent hébergées en France (RGPD)

### Qualité de code & Maintenabilité

- 100% code coverage sur le control plane Rust
- Aucun module ne dépasse 500 lignes de code ; la structure modulaire est définie lors de l'étape architecture
- Les .proto sont la source de vérité pour les contrats API : toute modification régénère les types Rust et TypeScript
- Chaque fonction publique est documentée ; les conventions de nommage et patterns sont documentés dans un guide d'architecture pour guider les agents IA

### Fiabilité

- Le provisionnement d'une application qui échoue ne laisse pas de ressources orphelines (Helm release, subscription Stripe)
- Les opérations Stripe et K8s sont idempotentes ou compensables en cas d'échec partiel
