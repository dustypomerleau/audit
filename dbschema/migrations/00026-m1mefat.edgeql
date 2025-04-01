CREATE MIGRATION m1mefatbmhqlx3wjqyp34yblm3adblxat6crx4jajiyw5hriybnv5a
    ONTO m1qomdxupywbav5edmof73vdy6le4qgwnrhcjbgri75pachugwemmq
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
  ALTER TYPE default::Surgeon {
      DROP LINK default_iol;
      DROP LINK default_site;
      DROP LINK cases;
      DROP LINK identity;
      DROP LINK sia;
      DROP PROPERTY email;
      DROP PROPERTY first_name;
      DROP PROPERTY last_name;
      DROP PROPERTY terms;
  };
  DROP TYPE default::Iol;
  DROP TYPE default::K;
  DROP TYPE default::OpRefraction;
  DROP TYPE default::OpVa;
  DROP TYPE default::Refraction;
  DROP TYPE default::Site;
  DROP TYPE default::SurgeonCas;
  DROP TYPE default::Surgeon;
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
  DROP SCALAR TYPE default::Kpower;
  DROP SCALAR TYPE default::Lt;
  DROP SCALAR TYPE default::Side;
  DROP SCALAR TYPE default::Wtw;
  DROP EXTENSION auth;
  DROP EXTENSION pgcrypto;
};
