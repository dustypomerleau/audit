CREATE MIGRATION m1ac6larv2ixcv2cco5frdahi6pr42hiwovkiqvkycm3ejcxrtu4oq
    ONTO m1ya5cthfrzl7sgmgkb4nx5fw23eh2vgabdiub4vi7ygvpynda7pga
{
  ALTER TYPE default::Surgeon {
      CREATE REQUIRED PROPERTY terms: std::bool {
          SET default := false;
      };
  };
};
