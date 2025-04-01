CREATE MIGRATION m1dczkgbyvxt4pfumk4h6pesqwihtkg6syowefp7ekku72oohpudfq
    ONTO m1nlerkkhr5vaimce3gj5gljsfghmbyxan3y3us6iruey4k3c3e7xq
{
  ALTER TYPE default::Site {
      DROP PROPERTY label;
  };
  ALTER TYPE default::Site {
      ALTER PROPERTY value {
          RENAME TO name;
      };
  };
};
