CREATE MIGRATION m1gsrh4p5innhxf2tab5tqvhjf3nyikei7xs3p7gmra2rsgvmzeolq
    ONTO m1nawfttfds2j5ojm2ae2hurmsb46dxrg6lz55nlunbcsxxoaz2p6q
{
  ALTER TYPE default::AfterVa {
      ALTER LINK best {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK raw {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::BeforeVa {
      ALTER LINK best {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK raw {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::Biometry {
      ALTER LINK ks {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::Cas {
      ALTER LINK biometry {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK iol {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK refraction {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK sia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK target {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK va {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::Ks {
      ALTER LINK flat {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK steep {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::OpRefraction {
      ALTER LINK after {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK before {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::OpVa {
      ALTER LINK after {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK before {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::Surgeon {
      ALTER LINK defaults {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK sia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::SurgeonCas {
      ALTER LINK cas {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK surgeon {
          ON TARGET DELETE DELETE SOURCE;
      };
  };
  ALTER TYPE default::SurgeonSia {
      ALTER LINK left {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      ALTER LINK right {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  ALTER TYPE default::Target {
      ALTER LINK cyl {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
};
