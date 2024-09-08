CREATE MIGRATION m135ho4g4aupkvl5xcu6l5dskpn44pwpfxrlbw7et3b2fgvrscp74q
    ONTO m13jvett5r5huyib6g3v3d3udm2745oixirbnvliiahlqhb3frqcya
{
  ALTER TYPE default::Cas {
      CREATE LINK opiol: default::OpIol;
  };
  CREATE TYPE default::SurgeonConstant EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK surgeon: default::Surgeon;
      CREATE REQUIRED LINK constant: default::Constant;
      CREATE REQUIRED LINK iol: default::Iol;
  };
  ALTER TYPE default::Surgeon {
      CREATE MULTI LINK constants := (.<surgeon[IS default::SurgeonConstant]);
  };
  ALTER TYPE default::SurgeonCas {
      DROP LINK iol;
  };
};
