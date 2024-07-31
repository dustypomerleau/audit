CREATE MIGRATION m1mxgky4tq7i7j23fe2vs4mm62ltylwj2ded5uyp345ksfibon7vwq
    ONTO m1gnnlqxuprgd2bjlvmlf6j7ejque4keu6d7r744wqvhop7y4cchwa
{
  CREATE GLOBAL default::cur_surgeon -> std::uuid;
  CREATE ABSTRACT TYPE default::SoftCreate {
      CREATE REQUIRED PROPERTY created_at: std::datetime {
          SET default := (std::datetime_current());
          SET readonly := true;
      };
  };
  CREATE TYPE default::Va EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY den: std::int32 {
          CREATE CONSTRAINT std::min_ex_value(0);
      };
      CREATE REQUIRED PROPERTY num: std::int32 {
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
  CREATE SCALAR TYPE default::Formula EXTENDING enum<Barrett, BarrettTrueK, Haigis, HofferQ, Holladay1, Holladay2, Kane, Olsen, SrkT>;
  CREATE TYPE default::Constant EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY formula: default::Formula;
      CREATE REQUIRED PROPERTY value: std::int32;
  };
  CREATE SCALAR TYPE default::Focus EXTENDING enum<Mono, Edof, Multi>;
  CREATE TYPE default::Iol EXTENDING default::SoftCreate {
      CREATE REQUIRED MULTI LINK constants: default::Constant;
      CREATE REQUIRED PROPERTY company: std::str;
      CREATE REQUIRED PROPERTY focus: default::Focus {
          SET default := (default::Focus.Mono);
      };
      CREATE REQUIRED PROPERTY model: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY name: std::str;
      CREATE REQUIRED PROPERTY toric: std::bool {
          SET default := false;
      };
  };
  CREATE SCALAR TYPE default::Axis EXTENDING std::int32 {
      CREATE CONSTRAINT std::max_value(179);
      CREATE CONSTRAINT std::min_value(0);
  };
  CREATE ABSTRACT TYPE default::Cyl {
      CREATE REQUIRED PROPERTY axis: default::Axis;
      CREATE REQUIRED PROPERTY power: std::int32;
  };
  CREATE TYPE default::IolCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON ((((.power >= 100) AND (.power <= 2000)) AND ((.power % 25) = 0)));
  };
  CREATE TYPE default::OpIol EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK iol: default::Iol;
      CREATE LINK cyl: default::IolCyl;
      CREATE REQUIRED PROPERTY se: std::int32 {
          CREATE CONSTRAINT std::expression ON (((__subject__ % 25) = 0));
          CREATE CONSTRAINT std::max_value(6000);
          CREATE CONSTRAINT std::min_value(-2000);
      };
  };
  CREATE TYPE default::RefractionCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON ((((.power >= -1000) AND (.power <= 1000)) AND ((.power % 25) = 0)));
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
  CREATE TYPE default::Sia EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON (((.power >= 0) AND (.power <= 200)));
  };
  CREATE TYPE default::TargetCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON (((.power >= 0) AND (.power <= 600)));
  };
  CREATE TYPE default::Target EXTENDING default::SoftCreate {
      CREATE LINK constant: default::Constant;
      CREATE LINK cyl: default::TargetCyl;
      CREATE REQUIRED PROPERTY se: std::int32 {
          CREATE CONSTRAINT std::max_value(200);
          CREATE CONSTRAINT std::min_value(-600);
      };
  };
  CREATE SCALAR TYPE default::Adverse EXTENDING enum<Rhexis, Pc, Zonule, Other>;
  CREATE SCALAR TYPE default::Side EXTENDING enum<Right, Left>;
  CREATE TYPE default::Cas EXTENDING default::SoftCreate {
      CREATE LINK iol: default::OpIol;
      CREATE REQUIRED LINK refraction: default::OpRefraction;
      CREATE LINK sia: default::Sia;
      CREATE LINK target: default::Target;
      CREATE REQUIRED LINK va: default::OpVa;
      CREATE PROPERTY adverse: default::Adverse;
      CREATE REQUIRED PROPERTY date: cal::local_date;
      CREATE REQUIRED PROPERTY side: default::Side;
      CREATE PROPERTY site: std::str;
      CREATE REQUIRED PROPERTY urn: std::str;
  };
  CREATE TYPE default::SurgeonSia EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK left: default::Sia;
      CREATE REQUIRED LINK right: default::Sia;
  };
  CREATE SCALAR TYPE default::EmailType EXTENDING std::str {
      CREATE CONSTRAINT std::regexp(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$");
  };
  CREATE TYPE default::Surgeon EXTENDING default::SoftCreate {
      CREATE LINK sia: default::SurgeonSia;
      CREATE REQUIRED PROPERTY email: default::EmailType {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY first_name: std::str;
      CREATE PROPERTY last_name: std::str;
      CREATE PROPERTY site: std::str;
  };
  ALTER TYPE default::Cas {
      CREATE REQUIRED LINK surgeon: default::Surgeon;
  };
  ALTER TYPE default::Surgeon {
      CREATE MULTI LINK cases := (.<surgeon[IS default::Cas]);
  };
};
