CREATE MIGRATION m1uevqofuupuys4wvf6rcl2gjzibwnwhmxzwawst3knpau4lh3yjza
    ONTO m1ull2b5wklyd2qsmcoqfimythantk623dengmeibwel6yo2zhz4vq
{
  ALTER TYPE default::Cas {
      DROP CONSTRAINT std::exclusive ON ((.urn, .side));
  };
  ALTER TYPE default::Cas {
      CREATE CONSTRAINT std::exclusive ON ((.surgeon, .urn, .side));
  };
  ALTER TYPE default::Constant {
      DROP CONSTRAINT std::exclusive ON ((.value, .formula));
  };
  ALTER TYPE default::IolCyl {
      DROP CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::OpIol {
      DROP CONSTRAINT std::exclusive ON ((.iol, .se, .cyl));
  };
  ALTER TYPE default::Refraction {
      DROP CONSTRAINT std::exclusive ON ((.sph, .cyl));
  };
  ALTER TYPE default::RefractionCyl {
      DROP CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Sia {
      DROP CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Target {
      DROP CONSTRAINT std::exclusive ON ((.constant, .se, .cyl));
  };
  ALTER TYPE default::TargetCyl {
      DROP CONSTRAINT std::exclusive ON ((.power, .axis));
  };
  ALTER TYPE default::Va {
      DROP CONSTRAINT std::exclusive ON ((.num, .den));
  };
};
