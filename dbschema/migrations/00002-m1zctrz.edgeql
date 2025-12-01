CREATE MIGRATION m1zctrzzfeea6r24hrcjg7soa73ylbgm5ll6sxeilhaxblyq75h5wa
    ONTO m1favi3rvyox7zwbrkxmenllcdhne7zokdtidwh7kzyk6v3atced3q
{
  CREATE SCALAR TYPE default::CasNumber EXTENDING std::sequence;
  ALTER TYPE default::SurgeonCas {
      DROP CONSTRAINT std::exclusive ON ((.surgeon, .urn, .side));
  };
  ALTER TYPE default::SurgeonCas {
      CREATE REQUIRED PROPERTY number: default::CasNumber {
          SET REQUIRED USING (<default::CasNumber>{(SELECT
              std::sequence_next(INTROSPECT default::CasNumber)
          )});
          CREATE CONSTRAINT std::exclusive;
      };
  };
  ALTER TYPE default::SurgeonCas {
      DROP PROPERTY urn;
  };
};
