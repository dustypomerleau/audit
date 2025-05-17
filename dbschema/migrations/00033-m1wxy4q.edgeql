CREATE MIGRATION m1wxy4qfs6kraw435yi6zy4dlhkgxdlz5wo4x2mtktqa6zfuoyhsla
    ONTO m1iwjufq2jwl3rspeldx454n6n32r767s4g6qinew2t6nchdsfefsa
{
  ALTER TYPE default::SurgeonCas {
      CREATE PROPERTY side := (.cas.side);
  };
};
