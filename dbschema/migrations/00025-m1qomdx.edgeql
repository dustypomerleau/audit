CREATE MIGRATION m1qomdxupywbav5edmof73vdy6le4qgwnrhcjbgri75pachugwemmq
    ONTO m1ihonrqx64bxrnokiaa47h4pxfeyfplpeh42ozzvifsuv2fttq66q
{
  ALTER TYPE default::Biometry {
      ALTER LINK flat_k_c {
          RENAME TO flat_k;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER LINK steep_k_c {
          RENAME TO steep_k;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER PROPERTY acd_c {
          RENAME TO acd;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER PROPERTY al_c {
          RENAME TO al;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER PROPERTY lt_c {
          RENAME TO lt;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER PROPERTY wtw_c {
          RENAME TO wtw;
      };
  };
  ALTER TYPE default::Cyl {
      ALTER PROPERTY power_c {
          RENAME TO power;
      };
  };
  ALTER TYPE default::K {
      ALTER PROPERTY power_c {
          RENAME TO power;
      };
  };
  ALTER TYPE default::OpIol {
      ALTER PROPERTY se_c {
          RENAME TO se;
      };
  };
  ALTER TYPE default::Target {
      ALTER PROPERTY se_c {
          RENAME TO se;
      };
  };
  ALTER TYPE default::Va {
      ALTER PROPERTY den_c {
          RENAME TO den;
      };
  };
  ALTER TYPE default::Va {
      ALTER PROPERTY num_c {
          RENAME TO num;
      };
  };
};
