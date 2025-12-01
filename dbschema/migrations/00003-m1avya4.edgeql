CREATE MIGRATION m1avya4bbifsqgv22eljsh4ud24g4rlt2czlxvfkppi2no3hypmxjq
    ONTO m1zctrzzfeea6r24hrcjg7soa73ylbgm5ll6sxeilhaxblyq75h5wa
{
  CREATE FUTURE simple_scoping;
  CREATE FUTURE warn_old_scoping;
  ALTER TYPE default::Surgeon {
      ALTER PROPERTY first_name {
          RENAME TO full_name;
      };
  };
  ALTER TYPE default::Surgeon {
      ALTER PROPERTY last_name {
          RENAME TO preferred_name;
      };
  };
};
