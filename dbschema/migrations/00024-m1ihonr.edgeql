CREATE MIGRATION m1ihonrqx64bxrnokiaa47h4pxfeyfplpeh42ozzvifsuv2fttq66q
    ONTO m1wsvv4neukt6ikzlsvjset77fdwljhtcpb262ulo2552i26vaggxa
{
  CREATE EXTENSION pgcrypto VERSION '1.3';
  CREATE EXTENSION auth VERSION '1.0';
  CREATE SCALAR TYPE default::Acd EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(600);
      CREATE CONSTRAINT std::min_value(0);
  };
  CREATE SCALAR TYPE default::Adverse EXTENDING enum<Rhexis, Pc, Zonule, Other>;
  CREATE SCALAR TYPE default::Al EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(3800);
      CREATE CONSTRAINT std::min_value(1200);
  };
  CREATE SCALAR TYPE default::Axis EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(179);
      CREATE CONSTRAINT std::min_value(0);
  };
  CREATE SCALAR TYPE default::Cct EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(650);
      CREATE CONSTRAINT std::min_value(350);
  };
  CREATE SCALAR TYPE default::EmailType EXTENDING std::str {
      CREATE CONSTRAINT std::regexp(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
  };
  CREATE SCALAR TYPE default::Focus EXTENDING enum<Mono, Edof, Multi>;
  CREATE SCALAR TYPE default::Formula EXTENDING enum<AscrsKrs, Barrett, BarrettTrueK, Evo, Haigis, HaigisL, HillRbf, HofferQ, Holladay1, Holladay2, Kane, Okulix, Olsen, SrkT>;
  CREATE SCALAR TYPE default::Kpower EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(6500);
      CREATE CONSTRAINT std::min_value(3000);
  };
  CREATE SCALAR TYPE default::Lt EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(800);
      CREATE CONSTRAINT std::min_value(200);
  };
  CREATE SCALAR TYPE default::Side EXTENDING enum<Right, Left>;
  CREATE SCALAR TYPE default::Wtw EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(1400);
      CREATE CONSTRAINT std::min_value(800);
  };
  CREATE ABSTRACT TYPE default::SoftCreate {
      CREATE REQUIRED PROPERTY created_at: std::datetime {
          SET default := (std::datetime_current());
          SET readonly := true;
      };
  };
  CREATE TYPE default::Iol EXTENDING default::SoftCreate {
      CREATE PROPERTY company: std::str;
      CREATE REQUIRED PROPERTY focus: default::Focus {
          SET default := (default::Focus.Mono);
      };
      CREATE REQUIRED PROPERTY model: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY name: std::str;
      CREATE REQUIRED PROPERTY toric: std::bool {
          SET default := false;
      };
  };
  CREATE TYPE default::Site EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::K EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power_c: default::Kpower;
  };
  CREATE TYPE default::Biometry EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK flat_k_c: default::K;
      CREATE REQUIRED LINK steep_k_c: default::K;
      CREATE REQUIRED PROPERTY acd_c: default::Acd;
      CREATE REQUIRED PROPERTY al_c: default::Al;
      CREATE PROPERTY cct: default::Cct;
      CREATE REQUIRED PROPERTY lt_c: default::Lt;
      CREATE PROPERTY wtw_c: default::Wtw;
  };
  CREATE ABSTRACT TYPE default::Cyl EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power_c: std::int32;
  };
  CREATE TYPE default::IolCyl EXTENDING default::Cyl {
      CREATE CONSTRAINT std::expression ON ((((.power_c >= 100) AND (.power_c <= 2000)) AND ((.power_c % 25) = 0)));
  };
  CREATE TYPE default::OpIol EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK iol: default::Iol;
      CREATE LINK cyl: default::IolCyl;
      CREATE REQUIRED PROPERTY se_c: std::int32 {
          CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
          CREATE CONSTRAINT std::max_value(6000);
          CREATE CONSTRAINT std::min_value(-2000);
      };
  };
  CREATE TYPE default::RefractionCyl EXTENDING default::Cyl {
      CREATE CONSTRAINT std::expression ON ((((.power_c >= -1000) AND (.power_c <= 1000)) AND ((.power_c % 25) = 0)));
  };
  CREATE TYPE default::Refraction EXTENDING default::SoftCreate {
      CREATE LINK cyl: default::RefractionCyl;
      CREATE REQUIRED PROPERTY sph: std::int32 {
          CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
          CREATE CONSTRAINT std::max_value(2000);
          CREATE CONSTRAINT std::min_value(-2000);
      };
  };
  CREATE TYPE default::OpRefraction EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::Refraction;
      CREATE REQUIRED LINK before: default::Refraction;
  };
  CREATE TYPE default::Va EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY den_c: std::int32 {
          CREATE CONSTRAINT std::min_ex_value(0);
      };
      CREATE REQUIRED PROPERTY num_c: std::int32 {
          CREATE CONSTRAINT std::max_value(2000);
          CREATE CONSTRAINT std::min_value(0);
      };
  };
  CREATE TYPE default::AfterVa EXTENDING default::SoftCreate {
      CREATE LINK best: default::Va;
      CREATE REQUIRED LINK raw: default::Va;
  };
  CREATE TYPE default::BeforeVa EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK best: default::Va;
      CREATE LINK raw: default::Va;
  };
  CREATE TYPE default::OpVa EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::AfterVa;
      CREATE REQUIRED LINK before: default::BeforeVa;
  };
  CREATE TYPE default::Sia EXTENDING default::Cyl {
      CREATE CONSTRAINT std::expression ON (((.power_c >= 0) AND (.power_c <= 200)));
  };
  CREATE TYPE default::TargetCyl EXTENDING default::Cyl {
      CREATE CONSTRAINT std::expression ON (((.power_c >= 0) AND (.power_c <= 600)));
  };
  CREATE TYPE default::Target EXTENDING default::SoftCreate {
      CREATE LINK cyl: default::TargetCyl;
      CREATE PROPERTY custom_constant: std::bool {
          SET default := false;
      };
      CREATE PROPERTY formula: default::Formula;
      CREATE REQUIRED PROPERTY se_c: std::int32 {
          CREATE CONSTRAINT std::max_value(200);
          CREATE CONSTRAINT std::min_value(-600);
      };
  };
  CREATE TYPE default::Cas EXTENDING default::SoftCreate {
      CREATE LINK biometry: default::Biometry;
      CREATE LINK opiol: default::OpIol;
      CREATE REQUIRED LINK refraction: default::OpRefraction;
      CREATE LINK sia: default::Sia;
      CREATE REQUIRED LINK target: default::Target;
      CREATE REQUIRED LINK va: default::OpVa;
      CREATE PROPERTY adverse: default::Adverse;
      CREATE REQUIRED PROPERTY side: default::Side;
      CREATE REQUIRED PROPERTY year: std::int32 {
          SET default := (<std::int32>std::datetime_get(std::datetime_current(), 'year'));
          CREATE CONSTRAINT std::max_value(2100);
          CREATE CONSTRAINT std::min_value(2000);
      };
  };
  CREATE TYPE default::SurgeonSia EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK left: default::Sia;
      CREATE REQUIRED LINK right: default::Sia;
  };
  CREATE TYPE default::Surgeon EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK identity: ext::auth::Identity {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE LINK default_iol: default::Iol;
      CREATE LINK default_site: default::Site;
      CREATE LINK sia: default::SurgeonSia;
      CREATE REQUIRED PROPERTY email: default::EmailType {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY first_name: std::str;
      CREATE PROPERTY last_name: std::str;
      CREATE PROPERTY terms: std::datetime;
  };
  CREATE TYPE default::SurgeonCas EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK surgeon: default::Surgeon;
      CREATE REQUIRED LINK cas: default::Cas {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE LINK site: default::Site;
      CREATE REQUIRED PROPERTY date: std::cal::local_date;
      CREATE REQUIRED PROPERTY urn: std::str;
  };
  ALTER TYPE default::Surgeon {
      CREATE MULTI LINK cases := (.<surgeon[IS default::SurgeonCas]);
  };
  CREATE GLOBAL default::cur_surgeon := (std::assert_single((SELECT
      default::Surgeon
  FILTER
      (.identity = GLOBAL ext::auth::ClientTokenIdentity)
  )));
  ALTER TYPE default::SurgeonCas {
      CREATE ACCESS POLICY surgeon_full_access
          ALLOW ALL USING ((.surgeon ?= GLOBAL default::cur_surgeon)) {
              SET errmessage := 'Only the surgeon has access to their cases.';
          };
  };
};
