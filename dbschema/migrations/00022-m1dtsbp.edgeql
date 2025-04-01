CREATE MIGRATION m1dtsbpeazsgc7kmd55cfync5rm6zk45lrszffbgxd2aksej56kciq
    ONTO m1pfh3cob6djqtentqssm2hz4remnanmg3pwr7gajsh565cvqnmnia
{
  ALTER TYPE default::Surgeon {
      CREATE LINK default_constant: default::SurgeonConstant;
  };
};
