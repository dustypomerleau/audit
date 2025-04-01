CREATE MIGRATION m1nlerkkhr5vaimce3gj5gljsfghmbyxan3y3us6iruey4k3c3e7xq
    ONTO m1mhp2r5dl3xmhyo6kqr3742bbcgkcgir4xilu22rhhpbo2vwpbnoa
{
  ALTER TYPE default::Site {
      CREATE REQUIRED PROPERTY label: std::str {
          SET REQUIRED USING (<std::str>'hello');
      };
  };
  ALTER TYPE default::Site {
      ALTER PROPERTY name {
          RENAME TO value;
      };
  };
};
