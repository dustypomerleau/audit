CREATE MIGRATION m1oxjeht42gkz6rmuywdofvcpi2mvyusvy35qeustvfezto3ubi6va
    ONTO m1pu3lqgui2reha742qyw6ff7m25nsvrngxmrrtygjmrluwsk4cuyq
{
  CREATE GLOBAL default::cur_surgeon_email -> std::str;
  ALTER GLOBAL default::cur_surgeon USING (SELECT
      default::Surgeon
  FILTER
      (.email = GLOBAL default::cur_surgeon_email)
  );
  DROP GLOBAL default::cur_surgeon_id;
};
