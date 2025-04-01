CREATE MIGRATION m1mhp2r5dl3xmhyo6kqr3742bbcgkcgir4xilu22rhhpbo2vwpbnoa
    ONTO m1roas7xrqrq5poflvoosyjkahfmwvmf5zyzzqpal2sevjngz4wxqa
{
  ALTER TYPE default::K {
      DROP PROPERTY power;
  };
  ALTER TYPE default::K {
      CREATE PROPERTY power: std::int32 {
          SET REQUIRED USING (<std::int32>44);
      };
      DROP EXTENDING default::SoftCreate;
      EXTENDING default::Cyl LAST;
      CREATE CONSTRAINT std::expression ON (((.power >= 3000) AND (.power <= 6500)));
      ALTER PROPERTY axis {
          RESET OPTIONALITY;
          DROP OWNED;
          RESET TYPE;
      };
  };
  ALTER TYPE default::K {
      ALTER PROPERTY power {
          RESET OPTIONALITY;
          DROP OWNED;
          RESET TYPE;
      };
  };
  DROP SCALAR TYPE default::Kpower;
};
