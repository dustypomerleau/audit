CREATE MIGRATION m1pfh3cob6djqtentqssm2hz4remnanmg3pwr7gajsh565cvqnmnia
    ONTO m1zjpjodsxzbt2ufyxukpsbvc6lw2wiognwpkpgjzc7vcxni3hmf7a
{
  ALTER TYPE default::Surgeon {
      ALTER LINK identity {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
