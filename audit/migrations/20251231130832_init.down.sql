drop trigger if exists cas_updated_at on cas;
drop trigger if exists iol_updated_at on iol;
drop trigger if exists site_updated_at on site;
drop trigger if exists surgeon_cas_updated_at on surgeon_cas;
drop trigger if exists surgeon_updated_at on surgeon;

drop function if exists updated_at;

drop table if exists cas;
drop table if exists iol;
drop table if exists site;
drop table if exists surgeon;
drop table if exists surgeon_cas;

drop type if exists adverse;
drop type if exists focus;
drop type if exists formula;
drop type if exists side;

drop domain if exists acd;
drop domain if exists al;
drop domain if exists axis;
drop domain if exists cct;
drop domain if exists email;
drop domain if exists iol_se;
drop domain if exists k_power;
drop domain if exists lt;
drop domain if exists main;
drop domain if exists ref_cyl_power;
drop domain if exists ref_sph;
drop domain if exists sia_power;
drop domain if exists target_cyl_power;
drop domain if exists target_se;
drop domain if exists toric_power;
drop domain if exists va_den;
drop domain if exists va_num;
drop domain if exists wtw;
drop domain if exists year;

drop extension if exists citext;
