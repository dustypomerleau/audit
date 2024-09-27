CREATE MIGRATION m1qjs5ro3yj3r3xzd6xwsqwcfrdftybykh3z526ebzpnlpd3r7xfnq
    ONTO m1ujgsr2x2o3xydohoyur36h4rdnxkmzum5o43xfk3vjdktnsuju7q
{
  ALTER TYPE default::Cas {
      DROP PROPERTY date;
  };
  ALTER TYPE default::Cas {
      CREATE REQUIRED PROPERTY year: std::int32 {
          SET default := (<std::int32>std::datetime_get(std::datetime_current(), 'year'));
      };
  };
  ALTER TYPE default::SurgeonCas {
      CREATE REQUIRED PROPERTY date: cal::local_date {
          SET REQUIRED USING (<cal::local_date>{});
      };
  };
};
