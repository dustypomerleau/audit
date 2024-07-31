CREATE MIGRATION m1ull2b5wklyd2qsmcoqfimythantk623dengmeibwel6yo2zhz4vq
    ONTO m1mxgky4tq7i7j23fe2vs4mm62ltylwj2ded5uyp345ksfibon7vwq
{
  ALTER TYPE default::Cas {
      CREATE CONSTRAINT std::exclusive ON ((.urn, .side));
  };
  ALTER TYPE default::Cas {
      DROP PROPERTY site;
  };
  CREATE TYPE default::Site EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::Cas {
      CREATE LINK site: default::Site;
  };
  ALTER TYPE default::Constant {
      CREATE CONSTRAINT std::exclusive ON ((.value, .formula));
  };
  ALTER TYPE default::IolCyl {
      CREATE CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::OpIol {
      CREATE CONSTRAINT std::exclusive ON ((.iol, .se, .cyl));
  };
  ALTER TYPE default::Refraction {
      CREATE CONSTRAINT std::exclusive ON ((.sph, .cyl));
  };
  ALTER TYPE default::RefractionCyl {
      CREATE CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Sia {
      CREATE CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Surgeon {
      DROP PROPERTY site;
  };
  ALTER TYPE default::Surgeon {
      CREATE LINK site: default::Site;
  };
  ALTER TYPE default::Target {
      CREATE CONSTRAINT std::exclusive ON ((.constant, .se, .cyl));
  };
  ALTER TYPE default::TargetCyl {
      CREATE CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Va {
      CREATE CONSTRAINT std::exclusive ON ((.num, .den));
  };
};
