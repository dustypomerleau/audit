CREATE MIGRATION m1ya5cthfrzl7sgmgkb4nx5fw23eh2vgabdiub4vi7ygvpynda7pga
    ONTO m1qjs5ro3yj3r3xzd6xwsqwcfrdftybykh3z526ebzpnlpd3r7xfnq
{
  ALTER TYPE default::Surgeon {
      ALTER LINK cases {
          RESET CARDINALITY;
      };
      ALTER LINK constants {
          RESET CARDINALITY;
      };
  };
  ALTER TYPE default::Surgeon {
      CREATE LINK default_site: default::Site;
  };
  ALTER TYPE default::Surgeon {
      DROP LINK sites;
  };
};
