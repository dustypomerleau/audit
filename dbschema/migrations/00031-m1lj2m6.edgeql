CREATE MIGRATION m1lj2m6pdyzvddl7dvv5s2chdngg2kuqlw6k25vnnouucpju4btrva
    ONTO m1dczkgbyvxt4pfumk4h6pesqwihtkg6syowefp7ekku72oohpudfq
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
  ALTER TYPE default::Biometry {
      DROP LINK flat_k;
      DROP LINK steep_k;
      DROP PROPERTY acd;
      DROP PROPERTY al;
      DROP PROPERTY cct;
      DROP PROPERTY lt;
      DROP PROPERTY wtw;
  };
  DROP TYPE default::Cas;
  DROP TYPE default::Biometry;
  ALTER TYPE default::Cyl {
      DROP PROPERTY axis;
  };
  ALTER TYPE default::IolCyl {
      DROP CONSTRAINT std::expression ON ((((.power >= 100) AND (.power <= 2000)) AND ((.power % 25) = 0)));
  };
  ALTER TYPE default::K {
      DROP CONSTRAINT std::expression ON (((.power >= 3000) AND (.power <= 6500)));
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
  DROP TYPE default::K;
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
  DROP TYPE default::Target;
  DROP TYPE default::TargetCyl;
  DROP TYPE default::Cyl;
  ALTER TYPE default::Iol {
      DROP PROPERTY company;
      DROP PROPERTY focus;
      DROP PROPERTY model;
      DROP PROPERTY name;
      DROP PROPERTY toric;
  };
  ALTER TYPE default::SurgeonDefaults {
      DROP LINK iol;
      DROP LINK site;
      DROP PROPERTY custom_constant;
      DROP PROPERTY formula;
  };
  DROP TYPE default::Iol;
  DROP TYPE default::OpRefraction;
  DROP TYPE default::OpVa;
  DROP TYPE default::Refraction;
  DROP TYPE default::Site;
  ALTER TYPE default::Surgeon {
      DROP LINK cases;
      DROP LINK defaults;
      DROP LINK identity;
      DROP LINK sia;
      DROP PROPERTY email;
      DROP PROPERTY first_name;
      DROP PROPERTY last_name;
      DROP PROPERTY terms;
  };
  DROP TYPE default::SurgeonCas;
  DROP TYPE default::Surgeon;
  DROP TYPE default::SurgeonDefaults;
  DROP TYPE default::SurgeonSia;
  DROP TYPE default::Va;
  DROP TYPE default::SoftCreate;
  DROP SCALAR TYPE default::Acd;
  DROP SCALAR TYPE default::Adverse;
  DROP SCALAR TYPE default::Al;
  DROP SCALAR TYPE default::Axis;
  DROP SCALAR TYPE default::Cct;
  DROP SCALAR TYPE default::EmailType;
  DROP SCALAR TYPE default::Focus;
  DROP SCALAR TYPE default::Formula;
  DROP SCALAR TYPE default::Lt;
  DROP SCALAR TYPE default::Side;
  DROP SCALAR TYPE default::Wtw;
  DROP EXTENSION auth;
  DROP EXTENSION pgcrypto;
};
