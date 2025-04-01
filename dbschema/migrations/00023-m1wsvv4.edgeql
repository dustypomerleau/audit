CREATE MIGRATION m1wsvv4neukt6ikzlsvjset77fdwljhtcpb262ulo2552i26vaggxa
    ONTO m1dtsbpeazsgc7kmd55cfync5rm6zk45lrszffbgxd2aksej56kciq
{
  ALTER TYPE default::SurgeonCas {
      DROP ACCESS POLICY surgeon_full_access;
      DROP LINK cas;
      DROP LINK site;
  };
  DROP GLOBAL default::cur_surgeon;
  ALTER TYPE default::SoftCreate {
      DROP PROPERTY created_at;
  };
  ALTER TYPE default::AfterVa {
      DROP LINK best;
      DROP LINK raw;
  };
  ALTER TYPE default::OpVa {
      DROP LINK after;
      DROP LINK before;
  };
  DROP TYPE default::AfterVa;
  DROP TYPE default::BeforeVa;
  DROP TYPE default::Cas;
  ALTER TYPE default::Constant {
      DROP PROPERTY formula;
      DROP PROPERTY value;
  };
  ALTER TYPE default::SurgeonConstant {
      DROP LINK constant;
      DROP LINK iol;
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
      DROP CONSTRAINT std::expression ON ((((.power >= 100) AND (.power <= 2000)) AND ((.power % 25) = 0)));
  };
  ALTER TYPE default::RefractionCyl {
      DROP CONSTRAINT std::expression ON ((((.power >= -1000) AND (.power <= 1000)) AND ((.power % 25) = 0)));
  };
  ALTER TYPE default::Sia {
      DROP CONSTRAINT std::expression ON (((.power >= 0) AND (.power <= 200)));
  };
  ALTER TYPE default::TargetCyl {
      DROP CONSTRAINT std::expression ON (((.power >= 0) AND (.power <= 600)));
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
  ALTER TYPE default::SurgeonSia {
      DROP LINK left;
      DROP LINK right;
  };
  DROP TYPE default::Sia;
  DROP TYPE default::TargetCyl;
  DROP TYPE default::Cyl;
  DROP TYPE default::Iol;
  DROP TYPE default::OpRefraction;
  DROP TYPE default::OpVa;
  DROP TYPE default::Refraction;
  ALTER TYPE default::Site {
      DROP PROPERTY name;
  };
  ALTER TYPE default::Surgeon {
      DROP LINK default_site;
      DROP LINK cases;
      DROP LINK constants;
      DROP LINK default_constant;
      DROP LINK identity;
      DROP LINK sia;
      DROP PROPERTY email;
      DROP PROPERTY first_name;
      DROP PROPERTY last_name;
      DROP PROPERTY terms;
  };
  DROP TYPE default::Site;
  DROP TYPE default::SurgeonCas;
  DROP TYPE default::SurgeonConstant;
  DROP TYPE default::Surgeon;
  DROP TYPE default::SurgeonSia;
  DROP TYPE default::Va;
  DROP TYPE default::SoftCreate;
  DROP SCALAR TYPE default::Adverse;
  DROP SCALAR TYPE default::Axis;
  DROP SCALAR TYPE default::EmailType;
  DROP SCALAR TYPE default::Focus;
  DROP SCALAR TYPE default::Formula;
  DROP SCALAR TYPE default::Side;
  DROP EXTENSION auth;
  DROP EXTENSION pgcrypto;
};
