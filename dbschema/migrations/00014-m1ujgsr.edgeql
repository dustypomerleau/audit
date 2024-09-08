CREATE MIGRATION m1ujgsr2x2o3xydohoyur36h4rdnxkmzum5o43xfk3vjdktnsuju7q
    ONTO m135ho4g4aupkvl5xcu6l5dskpn44pwpfxrlbw7et3b2fgvrscp74q
{
  ALTER TYPE default::Surgeon {
      DROP LINK site;
  };
  ALTER TYPE default::Surgeon {
      CREATE MULTI LINK sites: default::Site;
  };
};
