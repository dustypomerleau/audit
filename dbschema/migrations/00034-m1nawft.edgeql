CREATE MIGRATION m1nawfttfds2j5ojm2ae2hurmsb46dxrg6lz55nlunbcsxxoaz2p6q
    ONTO m1wxy4qfs6kraw435yi6zy4dlhkgxdlz5wo4x2mtktqa6zfuoyhsla
{
  ALTER TYPE default::SurgeonCas {
      ALTER PROPERTY side {
          RESET EXPRESSION;
          RESET CARDINALITY;
          SET REQUIRED;
          SET TYPE default::Side;
      };
      CREATE CONSTRAINT std::exclusive ON ((.surgeon, .urn, .side));
  };
};
