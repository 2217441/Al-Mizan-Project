# Database Administration Tasks

## Backlog

### High Importance
- [ ] **Establish Backup Strategy**
  - Define RPO/RTO requirements.
  - Create a script for `surreal export`.
  - Schedule automated backups (cron/systemd/job scheduler).
- [ ] **Production Database Verification**
  - Identify where the production database is hosted.
  - Verify access controls and security groups.
  - Ensure backups are enabled for production.
- [ ] **Schema Documentation**
  - Reverse engineer the full schema from `.surql` files.
  - Document field types and relationships in `SCHEMA.md`.

### Medium Importance
- [ ] **Performance Tuning**
  - Monitor query performance for common queries (`almizan-core`).
  - Add indexes if missing for high-frequency lookups (e.g., searching hadith by narrator).
- [ ] **Security Hardening**
  - specific audit of `DB_USER` roles/permissions.
  - Ensure SSL/TLS is enforced in production.

### Ongoing
- [ ] **Monitoring**
    - Track disk usage of `surreal_data`.
    - Monitor memory usage of `almizan-db` container.

## Completed
- [x] Initial Database Analysis (`DB_ANALYSIS.md`).
- [x] Agent Setup and Directory creation.
