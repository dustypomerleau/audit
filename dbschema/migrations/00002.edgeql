CREATE MIGRATION m1wg5h3noh57sev7vvtlbh7xsp7m2cxftfpr4zjhkjerscuchczeda
    ONTO m1wxyhs7cgg7uyntsy5wrmkwknveeedi4wc4rrxqhatcfdhfj7boqq
{
  ALTER TYPE default::BeforeVaSet {
      DROP LINK raw_near;
  };
};
