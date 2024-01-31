CREATE MIGRATION m1wxyhs7cgg7uyntsy5wrmkwknveeedi4wc4rrxqhatcfdhfj7boqq
    ONTO initial
{
  CREATE ABSTRACT TYPE default::SoftCreate {
      CREATE REQUIRED PROPERTY created_at: std::datetime {
          SET default := (std::datetime_current());
          SET readonly := true;
      };
  };
  CREATE ABSTRACT TYPE default::Va {
      CREATE REQUIRED PROPERTY den: std::float32 {
          CREATE CONSTRAINT std::min_ex_value(0.0);
      };
      CREATE REQUIRED PROPERTY num: std::float32 {
          CREATE CONSTRAINT std::max_value(20.0);
          CREATE CONSTRAINT std::min_ex_value(0.0);
      };
  };
  CREATE TYPE default::FarVa EXTENDING default::SoftCreate, default::Va;
  CREATE TYPE default::NearVa EXTENDING default::SoftCreate, default::Va;
  CREATE TYPE default::AfterVaSet EXTENDING default::SoftCreate {
      CREATE LINK best_far: default::FarVa;
      CREATE REQUIRED LINK raw_far: default::FarVa;
      CREATE LINK raw_near: default::NearVa;
  };
  CREATE TYPE default::BeforeVaSet EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK best_far: default::FarVa;
      CREATE LINK raw_far: default::FarVa;
      CREATE LINK raw_near: default::NearVa;
  };
  CREATE TYPE default::OpVa EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::AfterVaSet;
      CREATE REQUIRED LINK before: default::BeforeVaSet;
  };
  CREATE SCALAR TYPE default::Lens EXTENDING enum<Thick, Thin>;
  CREATE TYPE default::Formula EXTENDING default::SoftCreate {
      CREATE REQUIRED PROPERTY lens: default::Lens;
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::Constant EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK formula: default::Formula;
      CREATE REQUIRED PROPERTY value: std::float32;
  };
  CREATE SCALAR TYPE default::Focus EXTENDING enum<Mono, Edof, Multi>;
  CREATE TYPE default::Iol EXTENDING default::SoftCreate {
      CREATE REQUIRED MULTI LINK constants: default::Constant;
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
      CREATE REQUIRED PROPERTY power: std::float32;
  };
  CREATE TYPE default::IolCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON ((((.power >= 1.0) AND (.power <= 20.0)) AND ((.power % 0.25) = 0.0)));
  };
  CREATE TYPE default::OpIol EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK iol: default::Iol;
      CREATE LINK cyl: default::IolCyl;
      CREATE REQUIRED PROPERTY se: std::float32 {
          CREATE CONSTRAINT std::expression ON (((__subject__ % 0.25) = 0.0));
          CREATE CONSTRAINT std::max_value(60.0);
          CREATE CONSTRAINT std::min_value(-20.0);
      };
  };
  CREATE TYPE default::RefCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON ((((.power >= -10.0) AND (.power <= 10.0)) AND ((.power % 0.25) = 0.0)));
  };
  CREATE TYPE default::Refraction EXTENDING default::SoftCreate {
      CREATE LINK cyl: default::RefCyl;
      CREATE REQUIRED PROPERTY sph: std::float32 {
          CREATE CONSTRAINT std::expression ON (((__subject__ % 0.25) = 0.0));
          CREATE CONSTRAINT std::max_value(20.0);
          CREATE CONSTRAINT std::min_value(-20.0);
      };
  };
  CREATE TYPE default::OpRefraction EXTENDING default::SoftCreate {
      CREATE REQUIRED LINK after: default::Refraction;
      CREATE REQUIRED LINK before: default::Refraction;
  };
  CREATE TYPE default::Sia EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON (((.power >= 0.0) AND (.power <= 2.0)));
  };
  CREATE TYPE default::TargetCyl EXTENDING default::Cyl, default::SoftCreate {
      CREATE CONSTRAINT std::expression ON (((.power >= 0.0) AND (.power <= 6.0)));
  };
  CREATE TYPE default::Target EXTENDING default::SoftCreate {
      CREATE LINK constant: default::Constant;
      CREATE LINK cyl: default::TargetCyl;
      CREATE REQUIRED PROPERTY se: std::float32 {
          CREATE CONSTRAINT std::max_value(2.0);
          CREATE CONSTRAINT std::min_value(-6.0);
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
      CREATE PROPERTY handed: default::Side;
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
