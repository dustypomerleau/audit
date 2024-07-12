CREATE MIGRATION m1gqnwlvyqshlg5hlltqwod24th4qvdx7vpzpoqyekm5wiypwpzp5a
    ONTO m1wg5h3noh57sev7vvtlbh7xsp7m2cxftfpr4zjhkjerscuchczeda
{
  CREATE GLOBAL default::cur_user -> std::uuid;
  ALTER TYPE default::Formula {
      DROP PROPERTY lens;
  };
  ALTER TYPE default::Formula {
      CREATE REQUIRED PROPERTY thick_lens: std::bool {
          SET REQUIRED USING (true);
      };
  };
  ALTER TYPE default::Iol {
      CREATE REQUIRED PROPERTY company: std::str {
          SET REQUIRED USING ('change company');
      };
  };
  ALTER TYPE default::RefCyl RENAME TO default::RefractionCyl;
  DROP SCALAR TYPE default::Lens;
};
