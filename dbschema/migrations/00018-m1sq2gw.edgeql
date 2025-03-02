CREATE MIGRATION m1sq2gwagl7svce4nr3lrlqh7aftyqyc6wa3agwvvgo5mojp6v2kfq
    ONTO m1ac6larv2ixcv2cco5frdahi6pr42hiwovkiqvkycm3ejcxrtu4oq
{
  ALTER TYPE default::Surgeon {
      ALTER PROPERTY terms {
          RESET default;
          RESET OPTIONALITY;
          SET TYPE std::datetime USING (<std::datetime>{});
      };
  };
};
