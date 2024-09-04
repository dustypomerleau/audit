CREATE MIGRATION m1lzu5dnw3m22qlhdmixunqjzexlki4o3wi5tvzctffnkg2pv6f2ha
    ONTO m1tae4uxddnljj7p64xflg4zvuwmxdpdancc76mjguaq3ewmua6oeq
{
  ALTER TYPE default::Surgeon {
      CREATE REQUIRED LINK identity: ext::auth::Identity {
          SET REQUIRED USING (<ext::auth::Identity>{});
      };
  };
  CREATE TYPE default::SurgeonCas EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK surgeon: default::Surgeon;
      CREATE LINK cas: default::Cas {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE LINK iol: default::OpIol;
      CREATE LINK site: default::Site;
      CREATE REQUIRED PROPERTY urn: std::str;
  };
  ALTER TYPE default::Surgeon {
      ALTER LINK cases {
          USING (.<surgeon[IS default::SurgeonCas]);
      };
  };
  ALTER GLOBAL default::cur_surgeon USING (std::assert_single((SELECT
      default::Surgeon
  FILTER
      (.identity = GLOBAL ext::auth::ClientTokenIdentity)
  )));
  ALTER TYPE default::SurgeonCas {
      CREATE ACCESS POLICY surgeon_full_access
          ALLOW ALL USING ((.surgeon ?= GLOBAL default::cur_surgeon)) {
              SET errmessage := 'Only the surgeon has access to their cases.';
          };
  };
  DROP GLOBAL default::cur_surgeon_email;
  ALTER TYPE default::Cas {
      DROP CONSTRAINT std::exclusive ON ((.surgeon, .urn, .side));
      DROP LINK iol;
      DROP LINK site;
      DROP LINK surgeon;
      DROP PROPERTY urn;
  };
};
