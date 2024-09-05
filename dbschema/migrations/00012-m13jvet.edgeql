CREATE MIGRATION m13jvett5r5huyib6g3v3d3udm2745oixirbnvliiahlqhb3frqcya
    ONTO m1lzu5dnw3m22qlhdmixunqjzexlki4o3wi5tvzctffnkg2pv6f2ha
{
  ALTER TYPE default::SurgeonCas {
      ALTER LINK cas {
          SET REQUIRED USING (<default::Cas>{});
      };
  };
};
