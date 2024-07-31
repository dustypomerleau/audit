CREATE MIGRATION m1gnnlqxuprgd2bjlvmlf6j7ejque4keu6d7r744wqvhop7y4cchwa
    ONTO m1gqnwlvyqshlg5hlltqwod24th4qvdx7vpzpoqyekm5wiypwpzp5a
{
  DROP GLOBAL default::cur_user;
  ALTER TYPE default::SoftCreate {
      DROP PROPERTY created_at;
  };
  ALTER TYPE default::AfterVaSet {
      DROP LINK best_far;
      DROP LINK raw_far;
      DROP LINK raw_near;
  };
  ALTER TYPE default::OpVa {
      DROP LINK after;
      DROP LINK before;
  };
  DROP TYPE default::AfterVaSet;
  DROP TYPE default::BeforeVaSet;
  ALTER TYPE default::Cas {
      DROP LINK iol;
      DROP LINK refraction;
      DROP LINK sia;
  };
  ALTER TYPE default::Surgeon {
      DROP LINK cases;
      DROP LINK sia;
      DROP PROPERTY email;
      DROP PROPERTY first_name;
      DROP PROPERTY handed;
      DROP PROPERTY last_name;
      DROP PROPERTY site;
  };
  DROP TYPE default::Cas;
  ALTER TYPE default::Constant {
      DROP LINK formula;
      DROP PROPERTY value;
  };
  DROP TYPE default::Target;
  ALTER TYPE default::Iol {
      DROP LINK constants;
      DROP PROPERTY company;
      DROP PROPERTY focus;
      DROP PROPERTY model;
      DROP PROPERTY name;
      DROP PROPERTY toric;
  };
  DROP TYPE default::Constant;
  ALTER TYPE default::Cyl {
      DROP PROPERTY axis;
  };
  ALTER TYPE default::IolCyl {
      DROP CONSTRAINT std::expression ON ((((.power >= 1.0) AND (.power <= 20.0)) AND ((.power % 0.25) = 0.0)));
  };
  ALTER TYPE default::RefractionCyl {
      DROP CONSTRAINT std::expression ON ((((.power >= -10.0) AND (.power <= 10.0)) AND ((.power % 0.25) = 0.0)));
  };
  ALTER TYPE default::Sia {
      DROP CONSTRAINT std::expression ON (((.power >= 0.0) AND (.power <= 2.0)));
  };
  ALTER TYPE default::TargetCyl {
      DROP CONSTRAINT std::expression ON (((.power >= 0.0) AND (.power <= 6.0)));
  };
  ALTER TYPE default::Cyl {
      DROP PROPERTY power;
  };
  DROP TYPE default::OpIol;
  DROP TYPE default::IolCyl;
  ALTER TYPE default::Refraction {
      DROP LINK cyl;
      DROP PROPERTY sph;
  };
  DROP TYPE default::RefractionCyl;
  DROP TYPE default::SurgeonSia;
  DROP TYPE default::Sia;
  DROP TYPE default::TargetCyl;
  DROP TYPE default::Cyl;
  ALTER TYPE default::Va {
      DROP PROPERTY den;
      DROP PROPERTY num;
  };
  DROP TYPE default::FarVa;
  DROP TYPE default::Formula;
  DROP TYPE default::Iol;
  DROP TYPE default::NearVa;
  DROP TYPE default::OpRefraction;
  DROP TYPE default::OpVa;
  DROP TYPE default::Refraction;
  DROP TYPE default::Surgeon;
  DROP TYPE default::SoftCreate;
  DROP TYPE default::Va;
  DROP SCALAR TYPE default::Adverse;
  DROP SCALAR TYPE default::Axis;
  DROP SCALAR TYPE default::EmailType;
  DROP SCALAR TYPE default::Focus;
  DROP SCALAR TYPE default::Side;
};
