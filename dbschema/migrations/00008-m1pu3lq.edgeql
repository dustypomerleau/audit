CREATE MIGRATION m1pu3lqgui2reha742qyw6ff7m25nsvrngxmrrtygjmrluwsk4cuyq
    ONTO m1uevqofuupuys4wvf6rcl2gjzibwnwhmxzwawst3knpau4lh3yjza
{
  CREATE GLOBAL default::cur_surgeon_id -> std::uuid;
  ALTER GLOBAL default::cur_surgeon USING (SELECT
      default::Surgeon
  FILTER
      (.id = GLOBAL default::cur_surgeon_id)
  );
};
