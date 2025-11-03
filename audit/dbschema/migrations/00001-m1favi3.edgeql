CREATE MIGRATION m1favi3rvyox7zwbrkxmenllcdhne7zokdtidwh7kzyk6v3atced3q
    ONTO initial
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
  CREATE SCALAR TYPE default::Formula EXTENDING enum<AscrsKrs, Barrett, BarrettTrueK, Evo, Haigis, HaigisL, HillRbf, HofferQ, Holladay1, Holladay2, Kane, Okulix, Olsen, SrkT, Other>;
  CREATE SCALAR TYPE default::IolSe EXTENDING std::int32 {
      CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
      CREATE CONSTRAINT std::max_value(6000);
      CREATE CONSTRAINT std::min_value(-2000);
  };
  CREATE SCALAR TYPE default::Kpower EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(6500);
      CREATE CONSTRAINT std::min_value(3000);
  };
  CREATE SCALAR TYPE default::Lt EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(800);
      CREATE CONSTRAINT std::min_value(200);
  };
  CREATE SCALAR TYPE default::Main EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(600);
      CREATE CONSTRAINT std::min_value(100);
  };
  CREATE SCALAR TYPE default::RefCylPower EXTENDING std::int32 {
      CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
      CREATE CONSTRAINT std::max_value(1000);
      CREATE CONSTRAINT std::min_value(-1000);
  };
  CREATE SCALAR TYPE default::RefSph EXTENDING std::int32 {
      CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
      CREATE CONSTRAINT std::max_value(2000);
      CREATE CONSTRAINT std::min_value(-2000);
  };
  CREATE SCALAR TYPE default::SiaPower EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(200);
      CREATE CONSTRAINT std::min_value(0);
  };
  CREATE SCALAR TYPE default::Side EXTENDING enum<Right, Left>;
  CREATE SCALAR TYPE default::TargetCylPower EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(600);
      CREATE CONSTRAINT std::min_value(0);
  };
  CREATE SCALAR TYPE default::TargetSe EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(200);
      CREATE CONSTRAINT std::min_value(-600);
  };
  CREATE SCALAR TYPE default::ToricPower EXTENDING std::int32 {
      CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
      CREATE CONSTRAINT std::max_value(2000);
      CREATE CONSTRAINT std::min_value(100);
  };
  CREATE SCALAR TYPE default::VaDen EXTENDING std::int32 {
      CREATE CONSTRAINT std::min_value(1);
  };
  CREATE SCALAR TYPE default::VaNum EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(2000);
      CREATE CONSTRAINT std::min_value(0);
  };
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
  CREATE TYPE default::K EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power: default::Kpower;
  };
  CREATE TYPE default::Ks EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK flat: default::K {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK steep: default::K {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::Biometry EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK ks: default::Ks {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED PROPERTY acd: default::Acd;
      CREATE REQUIRED PROPERTY al: default::Al;
      CREATE PROPERTY cct: default::Cct;
      CREATE REQUIRED PROPERTY lt: default::Lt;
      CREATE PROPERTY wtw: default::Wtw;
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
      CREATE PROPERTY toric: default::ToricPower;
  };
  CREATE TYPE default::OpIol EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK iol: default::Iol;
      CREATE PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY se: default::IolSe;
  };
  CREATE TYPE default::RefCyl EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power: default::RefCylPower;
  };
  CREATE TYPE default::Refraction EXTENDING default::SoftCreate {
      CREATE LINK cyl: default::RefCyl;
      CREATE REQUIRED PROPERTY sph: default::RefSph;
  };
  CREATE TYPE default::OpRefraction EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::Refraction {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK before: default::Refraction {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::Va EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY den: default::VaDen;
      CREATE REQUIRED PROPERTY num: default::VaNum;
  };
  CREATE TYPE default::AfterVa EXTENDING default::SoftCreate {
      CREATE LINK best: default::Va {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK raw: default::Va {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::BeforeVa EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK best: default::Va {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE LINK raw: default::Va {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::OpVa EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::AfterVa {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK before: default::BeforeVa {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::Sia EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power: default::SiaPower;
  };
  CREATE TYPE default::TargetCyl EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power: default::TargetCylPower;
  };
  CREATE TYPE default::Target EXTENDING default::SoftCreate {
      CREATE LINK cyl: default::TargetCyl {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED PROPERTY custom_constant: std::bool {
          SET default := false;
      };
      CREATE PROPERTY formula: default::Formula;
      CREATE REQUIRED PROPERTY se: default::TargetSe;
  };
  CREATE TYPE default::Cas EXTENDING default::SoftCreate {
      CREATE LINK biometry: default::Biometry {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE LINK iol: default::OpIol {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK refraction: default::OpRefraction {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE LINK sia: default::Sia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK target: default::Target {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK va: default::OpVa {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE PROPERTY adverse: default::Adverse;
      CREATE PROPERTY main: default::Main;
      CREATE REQUIRED PROPERTY side: default::Side;
      CREATE REQUIRED PROPERTY year: std::int32 {
          SET default := (<std::int32>std::datetime_get(std::datetime_current(), 'year'));
          CREATE CONSTRAINT std::max_value(2100);
          CREATE CONSTRAINT std::min_value(2000);
      };
  };
  CREATE TYPE default::Site EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::SurgeonDefaults EXTENDING default::SoftCreate {
      CREATE LINK iol: default::Iol;
      CREATE LINK site: default::Site;
      CREATE REQUIRED PROPERTY custom_constant: std::bool {
          SET default := false;
      };
      CREATE PROPERTY formula: default::Formula;
      CREATE PROPERTY main: default::Main;
  };
  CREATE TYPE default::SurgeonSia EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK left: default::Sia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED LINK right: default::Sia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
  };
  CREATE TYPE default::Surgeon EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK identity: ext::auth::Identity {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE LINK defaults: default::SurgeonDefaults {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE LINK sia: default::SurgeonSia {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
      };
      CREATE REQUIRED PROPERTY email: default::EmailType {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY first_name: std::str;
      CREATE PROPERTY last_name: std::str;
      CREATE PROPERTY terms: std::datetime;
  };
  CREATE TYPE default::SurgeonCas EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK surgeon: default::Surgeon {
          ON TARGET DELETE DELETE SOURCE;
      };
      CREATE REQUIRED LINK cas: default::Cas {
          ON SOURCE DELETE DELETE TARGET IF ORPHAN;
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE LINK site: default::Site;
      CREATE REQUIRED PROPERTY side: default::Side;
      CREATE REQUIRED PROPERTY urn: std::str;
      CREATE CONSTRAINT std::exclusive ON ((.surgeon, .urn, .side));
      CREATE REQUIRED PROPERTY date: std::cal::local_date;
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
